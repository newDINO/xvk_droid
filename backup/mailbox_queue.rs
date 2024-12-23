use core::slice;
use std::{collections::VecDeque, ptr::null_mut, sync::{Arc, Condvar, Mutex}, thread};

use vulkan_core::{VkDevice, VkDeviceMemory, VkFence, VkImage, VkQueue, VkResult, VkResult_VK_INCOMPLETE, VkResult_VK_SUCCESS, VkSemaphore, VkSwapchainCreateInfoKHR};

use crate::{export::INITS, pfn::{VK_DESTROY_IMAGE, VK_FREE_MEMORY, VK_GET_DEVICE_QUEUE}};

use super::util::{create_swapchain_image, signal_semaphore_fence, transfer_semaphore_fence, PresentEngine};

pub struct SwapChainMailBox {
    device: VkDevice,
    shared: Arc<SharedObjects>,

    /// Because VkSemaphore and VkFence we get from vkAcquireNextImageKHR are opaque handles, 
    /// we have to use a queue to signal them.
    /// Is is its sole purpose.
    queue: VkQueue,

    image_memories: Vec<VkDeviceMemory>,
}

struct SharedObjects {
    mutables: Mutex<SharedMutables>,
    wait_for_main: Condvar,
    wait_for_finish: Condvar,
    images: Vec<usize>, // Vec<VkImage>
}

struct SharedMutables {
    empty_images: Vec<usize>,
    present_queue: VecDeque<PresentMsg>,
    living: bool,
}

struct PresentMsg {
    wait_semaphore_count: u32,
    p_wait_semaphores: usize, // *const const VkSemaphore
    image_index: u32,
}

impl SwapChainMailBox {
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

        let image_count = create_info.minImageCount.max(3) as usize;
        let mut images = Vec::with_capacity(image_count);
        let mut image_memories = Vec::with_capacity(image_count);
        for _ in 0..image_count {
            let (image, memory, image_is_ok) = unsafe {
                create_swapchain_image(device, create_info)
            };
            images.push(image as usize);
            image_memories.push(memory);
            is_ok &= image_is_ok;
        }

        let shared = Arc::new(
            SharedObjects {
                mutables: Mutex::new(SharedMutables {
                    empty_images: (0..image_count).into_iter().collect(),
                    present_queue: VecDeque::new(),
                    living: true,
                }),
                wait_for_finish: Condvar::new(),
                wait_for_main: Condvar::new(),
                images,
            }
        );
        let shared2 = shared.clone();
        
        let device_ptr = device as usize;
        let surface_ptr = create_info.surface as usize;
        let width = create_info.imageExtent.width;
        let height = create_info.imageExtent.height;
        thread::spawn(move || {
            let device = device_ptr as _;
            let present_engine = unsafe {PresentEngine::new(
                device,
                surface_ptr as _,
                width,
                height,
                &mut true,
            )};
            let mut queue = null_mut();
            unsafe {
                VK_GET_DEVICE_QUEUE.unwrap()(device, INITS.present_queue_family, 0, &mut queue);
            }

            loop {
                let next_msg = {
                    let mutables = shared2.mutables.lock().unwrap();
                    let mut mutables = shared2.wait_for_main.wait_while(mutables, |shared| {
                        shared.present_queue.is_empty() && shared.living
                    }).unwrap();
                    if !mutables.living {
                        break;
                    }
                    mutables.present_queue.pop_front().unwrap()
                }; // Drop the MutexGuard to unlock SharedObjects

                let image_index = next_msg.image_index as usize;
                unsafe {
                    present_engine.present_on_image(
                        queue,
                        shared2.images[image_index] as _,
                        next_msg.wait_semaphore_count,
                        next_msg.p_wait_semaphores as _,
                    );
                }
                
                let mut mutables = shared2.mutables.lock().unwrap();
                mutables.empty_images.push(image_index);
                shared2.wait_for_finish.notify_one();
            }
            println!("Thread dropped");
        });

        *result = is_ok;

        Self {
            device,
            shared,
            image_memories,
            queue,
        }
    }
    pub unsafe fn get_swapchain_images(
        &self,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut VkImage,
    ) -> VkResult {
        let count = self.shared.images.len() as u32;
        *p_swapchain_image_count = count;
        if p_swapchain_images.is_null() {
            VkResult_VK_SUCCESS
        } else {
            let swapchain_image_slice = slice::from_raw_parts_mut(p_swapchain_images, *p_swapchain_image_count as usize);
            for i in 0..swapchain_image_slice.len() {
                swapchain_image_slice[i] = self.shared.images[i] as _;
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
        let mut mutables = self.shared.mutables.lock().unwrap();
        if let Some(index) = mutables.empty_images.pop() {
            *image_index_ref = index as u32;
            unsafe {
                signal_semaphore_fence(self.queue, semaphore, fence)
            }
        } else if let Some(msg) = mutables.present_queue.pop_front() {
            *image_index_ref = msg.image_index as u32;
            unsafe {
                transfer_semaphore_fence(
                    self.queue,
                    msg.wait_semaphore_count,
                    msg.p_wait_semaphores as _,
                    semaphore,
                    fence,
                )
            }
        } else {
            let mut mutables = self.shared.wait_for_finish.wait_while(mutables, |mutables| {
                mutables.empty_images.is_empty()
            }).unwrap();
            *image_index_ref = mutables.empty_images.pop().unwrap() as u32;
            unsafe {
                signal_semaphore_fence(self.queue, semaphore, fence)
            }
        }
    }
    pub fn queue_present(
        &self,
        image_index: u32,
        wait_semaphore_count: u32,
        p_wait_semaphores: *const VkSemaphore,
    ) -> bool {
        let mut mutables = self.shared.mutables.lock().unwrap();
        mutables.present_queue.push_back(PresentMsg {
            wait_semaphore_count,
            p_wait_semaphores: p_wait_semaphores as _,
            image_index,
        });
        self.shared.wait_for_main.notify_one();
        true
    }
}

impl Drop for SwapChainMailBox {
    fn drop(&mut self) {
        let mut mutables = self.shared.mutables.lock().unwrap();
        mutables.living = false;
        self.shared.wait_for_main.notify_one();
        for i in 0..self.shared.images.len() {
            unsafe {
                VK_DESTROY_IMAGE.unwrap()(self.device, self.shared.images[i] as _, null_mut());
                VK_FREE_MEMORY.unwrap()(self.device, self.image_memories[i], null_mut());
            }
        }
        println!("SwapChainMailBox dropped");
    }
}