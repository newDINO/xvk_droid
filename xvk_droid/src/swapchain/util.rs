use core::slice;
use std::{mem::MaybeUninit, ptr::null_mut};

use vulkan_core::*;
use xcb::{x, Connection};
use crate::{export::{find_memory_type, VkIcdSurfaceXcb, INITS}, pfn::*};

pub unsafe fn create_swapchain_image(
    device: VkDevice,
    create_info: &VkSwapchainCreateInfoKHR,
) -> (VkImage, VkDeviceMemory, bool) {
    let mut is_ok = true;
    
    let width = create_info.imageExtent.width;
    let height = create_info.imageExtent.height;
    let image_create_info = VkImageCreateInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
        pNext: null_mut(),
        flags: 0,
        imageType: VkImageType_VK_IMAGE_TYPE_2D,
        format: create_info.imageFormat,
        extent: VkExtent3D {
            width,
            height,
            depth: 1,
        },
        mipLevels: 1,
        arrayLayers: create_info.imageArrayLayers,
        samples: VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT,
        tiling: VkImageTiling_VK_IMAGE_TILING_OPTIMAL,
        usage: create_info.imageUsage | VkImageUsageFlagBits_VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
        sharingMode: create_info.imageSharingMode,
        queueFamilyIndexCount: create_info.queueFamilyIndexCount,
        pQueueFamilyIndices: create_info.pQueueFamilyIndices,
        initialLayout: VkImageLayout_VK_IMAGE_LAYOUT_UNDEFINED,
    };
    let mut image = null_mut();
    is_ok &= VK_CREATE_IMAGE.unwrap()(device, &image_create_info, null_mut(), &mut image) == 0;

    let mut image_memory_requirement = MaybeUninit::uninit();
    VK_GET_IMAGE_MEMORY_REQUIREMENTS.unwrap()(device, image, image_memory_requirement.as_mut_ptr());
    let image_memory_requirement = image_memory_requirement.assume_init();
    let image_memory_index = find_memory_type(image_memory_requirement.memoryTypeBits, VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
    let image_memory_index = image_memory_index.unwrap_or_else(|| {
        println!("Failed to find required image memory type index, using 0");
        0
    });

    let image_memory_info = VkMemoryAllocateInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
        pNext: null_mut(),
        allocationSize: image_memory_requirement.size,
        memoryTypeIndex: image_memory_index,
    };
    let mut image_memory = null_mut();
    is_ok &= VK_ALLOCATE_MEMORY.unwrap()(device, &image_memory_info, null_mut(), &mut image_memory) == 0;

    is_ok &= VK_BIND_IMAGE_MEMORY.unwrap()(device, image, image_memory, 0) == 0;

    (image, image_memory, is_ok)
}

pub unsafe fn create_swapchain_buffer(
    device: VkDevice,
    buffer_size: u64,
) -> (VkBuffer, VkDeviceMemory, bool) {
    let mut is_ok = true;
    
    let buffer_info = VkBufferCreateInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
        pNext: null_mut(),
        flags: 0,
        size: buffer_size,
        usage: VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_DST_BIT,
        sharingMode: VkSharingMode_VK_SHARING_MODE_EXCLUSIVE,
        queueFamilyIndexCount: 0,
        pQueueFamilyIndices: [].as_ptr(),
    };
    let mut buffer = null_mut();
    is_ok &= VK_CREATE_BUFFER.unwrap()(device, &buffer_info, null_mut(), &mut buffer) == 0;

    let mut buffer_memory_requirement = MaybeUninit::uninit();
    VK_GET_BUFFER_MEMORY_REQUIREMENTS.unwrap()(device, buffer, buffer_memory_requirement.as_mut_ptr());
    let buffer_memory_requirement = buffer_memory_requirement.assume_init();
    let buffer_memory_index = find_memory_type(buffer_memory_requirement.memoryTypeBits, VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT);

    let buffer_memory_info = VkMemoryAllocateInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
        pNext: null_mut(),
        allocationSize: buffer_memory_requirement.size,
        memoryTypeIndex: buffer_memory_index.unwrap_or_else(|| {
            println!(
                "XVK_DROID: Failed to find a memory type support host visible memory, using 0."
            );
            0
        }),
    };
    let mut buffer_memory = null_mut();
    is_ok &= VK_ALLOCATE_MEMORY.unwrap()(device, &buffer_memory_info, null_mut(), &mut buffer_memory) == 0;

    is_ok &= VK_BIND_BUFFER_MEMORY.unwrap()(device, buffer, buffer_memory, 0) == 0;

    (buffer, buffer_memory, is_ok)
}

pub unsafe fn signal_semaphore_fence(
    queue: VkQueue,
    semaphore: VkSemaphore,
    fence: VkFence,
) -> VkResult {
    let submit_info = VkSubmitInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_SUBMIT_INFO,
        pNext: null_mut(),
        waitSemaphoreCount: 0,
        pWaitSemaphores: [].as_ptr(),
        pWaitDstStageMask: [].as_ptr(),
        signalSemaphoreCount: 1,
        pSignalSemaphores: &semaphore,
        commandBufferCount: 0,
        pCommandBuffers: [].as_ptr(),
    };

    VK_QUEUE_SUBMIT.unwrap()(queue, 1, &submit_info, fence)
}


pub struct CopyEngine {

    device: VkDevice,
    /// VkFence to wait for copy to buffer.
    fence: VkFence,

    width: u32,
    height: u32,

    command_pool: VkCommandPool,
    command_buffer: VkCommandBuffer,
}

impl CopyEngine {
    pub unsafe fn new(
        device: VkDevice,
        width: u32,
        height: u32,
        result: &mut bool,
    ) -> Self {
        let mut is_ok = true;

        let mut queue = null_mut();
        let family = INITS.present_queue_family;
        VK_GET_DEVICE_QUEUE.unwrap()(device, family, 0, &mut queue);

        let pool_info = VkCommandPoolCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: null_mut(),
            flags: VkCommandPoolCreateFlagBits_VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
            queueFamilyIndex: family,
        };
        let mut command_pool = null_mut();
        is_ok &= VK_CREATE_COMMAND_POOL.unwrap()(device, &pool_info, null_mut(), &mut command_pool) == 0;

        let command_buffer_info = VkCommandBufferAllocateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: null_mut(),
            commandPool: command_pool,
            level: VkCommandBufferLevel_VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            commandBufferCount: 1,
        };
        let mut command_buffer = null_mut();
        is_ok &= VK_ALLOCATE_COMMAND_BUFFERS.unwrap()(device, &command_buffer_info, &mut command_buffer) == 0;

        let fence_info = VkFenceCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
        };
        let mut fence = null_mut();
        is_ok &= VK_CREATE_FENCE.unwrap()(device, &fence_info, null_mut(), &mut fence) == 0;

        *result = is_ok;

        Self {
            device,
            fence,
            width,
            height,
            command_buffer,
            command_pool,
        }
    }
    pub unsafe fn copy_image_to_buffer(
        &self,
        queue: VkQueue,
        image: VkImage,
        wait_semaphore_count: u32,
        p_wait_semaphores: *const VkSemaphore,
        buffer: VkBuffer,
    ) -> bool {
        let mut is_ok = true;
        // 1. Copy image to buffer.
        is_ok &= VK_RESET_COMMAND_BUFFER.unwrap()(self.command_buffer, 0) == 0;

        let begin_info = VkCommandBufferBeginInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: null_mut(),
            flags: 0,
            pInheritanceInfo: null_mut(),
        };
        is_ok &= VK_BEGIN_COMMAND_BUFFER.unwrap()(self.command_buffer, &begin_info) == 0;

        let barrier = VkImageMemoryBarrier {
            sType: VkStructureType_VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
            pNext: null_mut(),
            srcAccessMask: 0,
            dstAccessMask: VkAccessFlagBits_VK_ACCESS_TRANSFER_READ_BIT,
            oldLayout: VkImageLayout_VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
            newLayout: VkImageLayout_VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            srcQueueFamilyIndex: INITS.present_queue_family,
            dstQueueFamilyIndex: INITS.present_queue_family,
            image,
            subresourceRange: VkImageSubresourceRange {
                aspectMask: VkImageAspectFlagBits_VK_IMAGE_ASPECT_COLOR_BIT,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            },
        };
        VK_CMD_PIPELINE_BARRIER.unwrap()(
            self.command_buffer,
            VkPipelineStageFlagBits_VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT,
            VkPipelineStageFlagBits_VK_PIPELINE_STAGE_TRANSFER_BIT,
            0,
            0,
            null_mut(),
            0,
            null_mut(),
            1,
            &barrier,
        );
        let copy_region = VkBufferImageCopy {
            bufferOffset: 0,
            bufferRowLength: 0,
            bufferImageHeight: 0,
            imageSubresource: VkImageSubresourceLayers {
                aspectMask: VkImageAspectFlagBits_VK_IMAGE_ASPECT_COLOR_BIT,
                mipLevel: 0,
                baseArrayLayer: 0,
                layerCount: 1,
            },
            imageOffset: VkOffset3D { x: 0, y: 0, z: 0 },
            imageExtent: VkExtent3D {
                width: self.width,
                height: self.height,
                depth: 1,
            },
        };
        VK_CMD_COPY_IMAGE_TO_BUFFER.unwrap()(
            self.command_buffer,
            image,
            VkImageLayout_VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            buffer,
            1,
            &copy_region,
        );

        is_ok &= VK_END_COMMAND_BUFFER.unwrap()(self.command_buffer) == 0;

        let stage = VkPipelineStageFlagBits_VK_PIPELINE_STAGE_TRANSFER_BIT;
        let stages = vec![stage; wait_semaphore_count as usize];

        let submit_info = VkSubmitInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: null_mut(),
            waitSemaphoreCount: wait_semaphore_count,
            pWaitSemaphores: p_wait_semaphores,
            pWaitDstStageMask: stages.as_ptr(),
            signalSemaphoreCount: 0,
            pSignalSemaphores: [].as_ptr(),
            commandBufferCount: 1,
            pCommandBuffers: &self.command_buffer,
        };
        is_ok &= VK_QUEUE_SUBMIT.unwrap()(queue, 1, &submit_info, self.fence) == 0;

        is_ok &=
            VK_WAIT_FOR_FENCES.unwrap()(self.device, 1, &self.fence, 1, u64::MAX) == 0;
        is_ok &= VK_RESET_FENCES.unwrap()(self.device, 1, &self.fence) == 0;

        // 2. Map buffer and draw the content.
        is_ok
    }
}

impl Drop for CopyEngine {
    fn drop(&mut self) {
        unsafe {
            VK_FREE_COMMAND_BUFFERS.unwrap()(self.device, self.command_pool, 1, [self.command_buffer].as_ptr());
            VK_DESTROY_COMMAND_POOL.unwrap()(self.device, self.command_pool, null_mut());
            VK_DESTROY_FENCE.unwrap()(self.device, self.fence, null_mut());
        }
    }
}

pub unsafe fn present_buffer(
    device: VkDevice,
    width: u64,
    height: u64,
    surface: *const VkIcdSurfaceXcb,
    memory: VkDeviceMemory,
) -> bool {
    let mut is_ok = true;
    
    let size = width * height * 4;
    let mut data = null_mut();
    is_ok &= VK_MAP_MEMORY.unwrap()(
        device,
        memory,
        0,
        size as u64,
        0,
        &mut data,
    ) == 0;

    let surface = &*surface;
    let connection = Connection::from_raw_conn(surface.connection);

    let gc: x::Gcontext = connection.generate_id();
    let cookie = connection.send_request_checked(&x::CreateGc {
        cid: gc,
        drawable: x::Drawable::Window(surface.window),
        value_list: &[],
    });
    is_ok &= connection.check_request(cookie).is_ok();

    let cookie = connection.send_request_checked(&x::PutImage {
        format: x::ImageFormat::ZPixmap,
        drawable: x::Drawable::Window(surface.window),
        gc,
        width: width as u16,
        height: height as u16,
        dst_x: 0,
        dst_y: 0,
        left_pad: 0,
        depth: 24,
        data: slice::from_raw_parts(data as *mut u8, size as usize),
    });
    is_ok &= connection.check_request(cookie).is_ok();

    let cookie = connection.send_request_checked(&x::FreeGc { gc });
    is_ok &= connection.check_request(cookie).is_ok();

    is_ok &= connection.flush().is_ok();

    connection.into_raw_conn(); // Important, dropping the connection will disconnect the connection.

    VK_UNMAP_MEMORY.unwrap()(device, memory);

    is_ok
}
