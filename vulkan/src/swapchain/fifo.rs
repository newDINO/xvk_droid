use std::ptr::null_mut;

use vulkan_core::{VkBuffer, VkDevice, VkDeviceMemory, VkFence, VkImage, VkQueue, VkResult, VkResult_VK_INCOMPLETE, VkResult_VK_SUCCESS, VkSemaphore, VkSwapchainCreateInfoKHR};

use crate::{export::{VkIcdSurfaceXcb, INITS}, pfn::{VK_DESTROY_BUFFER, VK_DESTROY_IMAGE, VK_FREE_MEMORY, VK_GET_DEVICE_QUEUE}};

use super::util::{create_swapchain_buffer, create_swapchain_image, present_buffer, signal_semaphore_fence, CopyEngine};

/// Current implementation is not very "cuncurrent".
pub struct SwapChainFIFO {
    device: VkDevice,
    copy_engine: CopyEngine,

    /// Because VkSemaphore and VkFence we get from vkAcquireNextImageKHR are opaque handles, 
    /// we have to use a queue to signal them.
    /// Is is its sole purpose.
    queue: VkQueue,

    width: u64,
    height: u64,
    surface: *const VkIcdSurfaceXcb,

    image: VkImage,
    image_memory: VkDeviceMemory,
    buffer: VkBuffer,
    buffer_memory: VkDeviceMemory,
}

impl SwapChainFIFO {
    pub unsafe fn new(
        device: VkDevice,
        create_info: &VkSwapchainCreateInfoKHR,
        result: &mut bool,
    ) -> Self {
        let mut is_ok = true;

        let width = create_info.imageExtent.width;
        let height = create_info.imageExtent.height;

        let (image, image_memory, image_is_ok) = create_swapchain_image(device, create_info);
        is_ok &= image_is_ok;

        let mut copy_engine_is_ok = true;
        let copy_engine = CopyEngine::new(
            device,
            width,
            height,
            &mut copy_engine_is_ok
        );
        is_ok &= copy_engine_is_ok;

        let buffer_size = (width * height * 4) as u64;
        let (buffer, buffer_memory, buffer_is_ok) = create_swapchain_buffer(device, buffer_size);
        is_ok &= buffer_is_ok;

        let mut queue = null_mut();
        VK_GET_DEVICE_QUEUE.unwrap()(device, INITS.present_queue_family, 0, &mut queue);

        *result = is_ok;

        SwapChainFIFO {
            device,
            queue,
            copy_engine,
            surface: create_info.surface as _,
            width: width as u64,
            height: height as u64,
            image,
            image_memory,
            buffer,
            buffer_memory,
        }
    }
    pub unsafe fn get_swapchain_images(
        &self,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut VkImage,
    ) -> VkResult {
        if p_swapchain_images.is_null() {
            *p_swapchain_image_count = 1;
            VkResult_VK_SUCCESS
        } else {
            if *p_swapchain_image_count < 1 {
                VkResult_VK_INCOMPLETE
            } else {
                *p_swapchain_images = self.image;
                VkResult_VK_SUCCESS
            }
        }
    }
    pub unsafe fn acquire_next_image(
        &self,
        semaphore: VkSemaphore,
        fence: VkFence,
        p_image_index: *mut u32,
    ) -> VkResult {
        *p_image_index = 0;
        signal_semaphore_fence(self.queue, semaphore, fence)
    }
    pub unsafe fn queue_present(
        &self,
        queue: VkQueue,
        wait_semaphore_count: u32,
        p_wait_semaphores: *const VkSemaphore,
    ) -> bool {
        let mut is_ok = true;
        is_ok &= self.copy_engine.copy_image_to_buffer(
            queue,
            self.image,
            wait_semaphore_count,
            p_wait_semaphores,
            self.buffer,
        );
        is_ok &= present_buffer(
            self.device,
            self.width,
            self.height,
            self.surface,
            self.buffer_memory
        );
        is_ok
    }
}

impl Drop for SwapChainFIFO {
    fn drop(&mut self) {
        unsafe {
            VK_DESTROY_IMAGE.unwrap()(self.device, self.image, null_mut());
            VK_FREE_MEMORY.unwrap()(self.device, self.image_memory, null_mut());
            VK_DESTROY_BUFFER.unwrap()(self.device, self.buffer, null_mut());
            VK_FREE_MEMORY.unwrap()(self.device, self.buffer_memory, null_mut());
        }
    }
}