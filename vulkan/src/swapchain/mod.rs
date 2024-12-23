use fifo::SwapChainFIFO;
use mailbox::SwapChainMailBox;
use vulkan_core::{VkDevice, VkFence, VkImage, VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR, VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR, VkQueue, VkResult, VkSemaphore, VkSwapchainCreateInfoKHR};

mod util;

mod fifo;
mod mailbox;

pub enum SwapChain {
    MailBox(SwapChainMailBox),
    FIFO(SwapChainFIFO),
}

impl SwapChain {
    #[allow(non_upper_case_globals)]
    pub unsafe fn new(
        device: VkDevice,
        create_info: &VkSwapchainCreateInfoKHR,
        result: &mut bool,
    ) -> Self {
        match create_info.presentMode {
            VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR => Self::FIFO(SwapChainFIFO::new(device, create_info, result)),
            VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR => Self::MailBox(SwapChainMailBox::new(device, create_info, result)),
            _ => {
                println!("presentMode for swapchain creation not supported, using supported one");
                Self::FIFO(SwapChainFIFO::new(device, create_info, result))
            }
        }
    }
    pub unsafe fn get_swapchain_images(
        &self,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut VkImage,
    ) -> VkResult {
        match self {
            Self::FIFO(swapchain) => swapchain.get_swapchain_images(p_swapchain_image_count, p_swapchain_images),
            Self::MailBox(swapchain) => swapchain.get_swapchain_images(p_swapchain_image_count, p_swapchain_images),
        }
    }
    pub unsafe fn acquire_next_image(
        &self,
        semaphore: VkSemaphore,
        fence: VkFence,
        p_image_index: *mut u32,
    ) -> VkResult {
        match self {
            Self::FIFO(swapchain) => swapchain.acquire_next_image(semaphore, fence, p_image_index),
            Self::MailBox(swapchain) => swapchain.acquire_next_image(semaphore, fence, p_image_index),
        }
    }
    pub unsafe fn queue_present(
        &self,
        queue: VkQueue,
        image_index: u32,
        wait_semaphore_count: u32,
        p_wait_semaphores: *const VkSemaphore,
    ) -> bool {
        match self {
            Self::FIFO(swapchain) => swapchain.queue_present(queue, wait_semaphore_count, p_wait_semaphores),
            Self::MailBox(swapchain) => swapchain.queue_present(queue, image_index, wait_semaphore_count, p_wait_semaphores),
        }
    }
}
