use core::slice;
use std::{collections::VecDeque, ptr::null_mut, sync::{Arc, Condvar, Mutex}, thread};

use vulkan_core::{VkBuffer, VkDevice, VkDeviceMemory, VkFence, VkImage, VkQueue, VkResult, VkResult_VK_INCOMPLETE, VkResult_VK_SUCCESS, VkSemaphore, VkSwapchainCreateInfoKHR};

use crate::{export::INITS, pfn::{VK_DESTROY_BUFFER, VK_DESTROY_IMAGE, VK_FREE_MEMORY, VK_GET_DEVICE_QUEUE}};

use super::util::{create_swapchain_buffer, create_swapchain_image, present_buffer, signal_semaphore_fence, CopyEngine};

/// Because queue can only used in one thread, 
/// Copy Image to Buffer can is done on the main thread.
pub struct SwapChainFIFO {
    device: VkDevice,
    queue: VkQueue,
    copy_engine: CopyEngine,

    images: Vec<VkImage>,
    image_memories: Vec<VkDeviceMemory>,

    buffers: Vec<VkBuffer>,
    shared: Arc<Shared>,
}

pub struct Shared {
    mutable: Mutex<Mutable>,
    wait_for_thread: Condvar,
    wait_for_main: Condvar,
    memories: Vec<usize>, // *const VkDeviceMemory
}

pub struct Mutable {
    finished: Vec<usize>,
    queue: VecDeque<usize>,
    living: bool,
}

impl SwapChainFIFO {
    pub fn new(
        device: VkDevice,
        create_info: &VkSwapchainCreateInfoKHR,
        result: &mut bool,
    ) -> Self {
        let mut is_ok = true;
        let mut queue = null_mut();
        unsafe {
            VK_GET_DEVICE_QUEUE.unwrap()(device, INITS.present_queue_family, 0, &mut queue);
        }

        let width = create_info.imageExtent.width;
        let height = create_info.imageExtent.height;
        let buffer_size = (width * height * 4) as u64;

        let mut copy_engine_result = true;
        let copy_engine = unsafe {CopyEngine::new(
            device,
            width,
            height,
            &mut copy_engine_result,
        )};
        is_ok &= copy_engine_result;

        let count = create_info.minImageCount.max(3) as usize;
        
        let mut images = Vec::with_capacity(count);
        let mut buffers = Vec::with_capacity(count);
        let mut image_memories = Vec::with_capacity(count);
        let mut buffer_memories = Vec::with_capacity(count);
        for _ in 0..count {
            let (image, image_memory, image_is_ok) = unsafe {
                create_swapchain_image(device, create_info)
            };
            let (buffer, buffer_memory, buffer_is_ok) = unsafe {
                create_swapchain_buffer(device, buffer_size)
            };
            images.push(image);
            image_memories.push(image_memory);
            buffers.push(buffer);
            buffer_memories.push(buffer_memory as usize);
            is_ok = is_ok & image_is_ok & buffer_is_ok;
        }

        let shared = Arc::new(Shared {
            mutable: Mutex::new(Mutable {
                finished: (0..count).into_iter().collect(),
                queue: VecDeque::new(),
                living: true,
            }),
            wait_for_main: Condvar::new(),
            wait_for_thread: Condvar::new(),
            memories: buffer_memories
        });
        let shared2 = shared.clone();

        let device_ptr = device as usize;
        let surface_ptr = create_info.surface as usize;
        thread::spawn(move || {
            loop {
                let next_index = {
                    let mutable = shared2.mutable.lock().unwrap();
                    let mut mutable = shared2.wait_for_main.wait_while(mutable, |mutable| {
                        mutable.queue.is_empty() && mutable.living
                    }).unwrap();
                    if !mutable.living {
                        break;
                    }
                    mutable.queue.pop_front().unwrap()
                };
                unsafe {
                    present_buffer(
                        device_ptr as _,
                        width as u64,
                        height as u64,
                        surface_ptr as _,
                        shared2.memories[next_index] as _
                    );
                }
                shared2.mutable.lock().unwrap().finished.push(next_index);
                shared2.wait_for_thread.notify_one();
            }
        });

        *result = is_ok;

        Self {
            device,
            queue,
            copy_engine,
            images,
            image_memories,
            buffers,
            shared,
        }
    }
    pub unsafe fn get_swapchain_images(
        &self,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut VkImage,
    ) -> VkResult {
        let count = self.images.len() as u32;
        *p_swapchain_image_count = count;
        if p_swapchain_images.is_null() {
            VkResult_VK_SUCCESS
        } else {
            let swapchain_image_slice = slice::from_raw_parts_mut(p_swapchain_images, *p_swapchain_image_count as usize);
            for i in 0..swapchain_image_slice.len() {
                swapchain_image_slice[i] = self.images[i] as _;
            }
            if *p_swapchain_image_count < count {
                VkResult_VK_INCOMPLETE
            } else {
                VkResult_VK_SUCCESS
            }
        }
    }
    pub fn acquire_next_image(
        &self,
        semaphore: VkSemaphore,
        fence: VkFence,
        p_image_index: *mut u32,
    ) -> VkResult {
        let image_index_ref = unsafe {
            &mut *p_image_index
        };

        let mutable = self.shared.mutable.lock().unwrap();
        let mut mutable = self.shared.wait_for_thread.wait_while(mutable, |mutable| {
            mutable.finished.is_empty()
        }).unwrap();
        *image_index_ref = mutable.finished.pop().unwrap() as u32;

        unsafe {
            signal_semaphore_fence(self.queue, semaphore, fence)
        }
    }
    pub fn queue_present(
        &self,
        queue: VkQueue,
        image_index: u32,
        wait_semaphore_count: u32,
        p_wait_semaphores: *const VkSemaphore,
    ) -> bool {
        let mut is_ok = true;
        let index = image_index as usize;
        unsafe {
            is_ok &= self.copy_engine.copy_image_to_buffer(
                queue,
                self.images[index],
                wait_semaphore_count,
                p_wait_semaphores,
                self.buffers[index],
            )
        };
        self.shared.mutable.lock().unwrap().queue.push_back(index);
        self.shared.wait_for_main.notify_one();

        is_ok
    }
}

impl Drop for SwapChainFIFO {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.images.len() {
                VK_DESTROY_IMAGE.unwrap()(self.device, self.images[i], null_mut());
                VK_FREE_MEMORY.unwrap()(self.device, self.image_memories[i], null_mut());
                VK_DESTROY_BUFFER.unwrap()(self.device, self.buffers[i], null_mut());
                VK_FREE_MEMORY.unwrap()(self.device, self.shared.memories[i] as _, null_mut());
            }
        }
    }
}