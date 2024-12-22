use core::slice;
use std::mem::{transmute, MaybeUninit};
use std::ptr::null_mut;
use std::{ffi::CStr, sync::LazyLock};

use vulkan_core::{VkAccessFlagBits_VK_ACCESS_TRANSFER_READ_BIT, VkBufferCreateInfo, VkBufferImageCopy, VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_DST_BIT, VkCommandBufferAllocateInfo, VkCommandBufferBeginInfo, VkCommandBufferLevel_VK_COMMAND_BUFFER_LEVEL_PRIMARY, VkCommandPoolCreateFlagBits_VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT, VkCommandPoolCreateInfo, VkExtent3D, VkFenceCreateInfo, VkImageAspectFlagBits_VK_IMAGE_ASPECT_COLOR_BIT, VkImageCreateInfo, VkImageLayout_VK_IMAGE_LAYOUT_PRESENT_SRC_KHR, VkImageLayout_VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL, VkImageLayout_VK_IMAGE_LAYOUT_UNDEFINED, VkImageMemoryBarrier, VkImageSubresourceLayers, VkImageSubresourceRange, VkImageTiling_VK_IMAGE_TILING_OPTIMAL, VkImageType_VK_IMAGE_TYPE_2D, VkImageUsageFlagBits_VK_IMAGE_USAGE_TRANSFER_SRC_BIT, VkMemoryAllocateInfo, VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT, VkOffset3D, VkPipelineStageFlagBits_VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT, VkPipelineStageFlagBits_VK_PIPELINE_STAGE_TRANSFER_BIT, VkPresentInfoKHR, VkResult_VK_INCOMPLETE, VkResult_VK_SUCCESS, VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT, VkSemaphore, VkSharingMode_VK_SHARING_MODE_EXCLUSIVE, VkStructureType_VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO, VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO, VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO, VkStructureType_VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO, VkStructureType_VK_STRUCTURE_TYPE_FENCE_CREATE_INFO, VkStructureType_VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO, VkStructureType_VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER, VkStructureType_VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO, VkStructureType_VK_STRUCTURE_TYPE_SUBMIT_INFO, VkSubmitInfo};
use vulkan_core::{PFN_vkVoidFunction, VkAllocationCallbacks, VkBuffer, VkCommandBuffer, VkCommandPool, VkDevice, VkDeviceMemory, VkFence, VkImage, VkQueue, VkResult, VkSwapchainCreateInfoKHR, VkSwapchainKHR};
use xcb::{x, Connection};

use crate::pfn::{VK_ALLOCATE_COMMAND_BUFFERS, VK_ALLOCATE_MEMORY, VK_BEGIN_COMMAND_BUFFER, VK_BIND_BUFFER_MEMORY, VK_BIND_IMAGE_MEMORY, VK_CMD_COPY_IMAGE_TO_BUFFER, VK_CMD_PIPELINE_BARRIER, VK_CREATE_BUFFER, VK_CREATE_COMMAND_POOL, VK_CREATE_FENCE, VK_CREATE_IMAGE, VK_DESTROY_BUFFER, VK_DESTROY_COMMAND_POOL, VK_DESTROY_FENCE, VK_DESTROY_IMAGE, VK_END_COMMAND_BUFFER, VK_FREE_MEMORY, VK_GET_BUFFER_MEMORY_REQUIREMENTS, VK_GET_DEVICE_PROC_ADDR, VK_GET_DEVICE_QUEUE, VK_GET_IMAGE_MEMORY_REQUIREMENTS, VK_MAP_MEMORY, VK_QUEUE_SUBMIT, VK_RESET_COMMAND_BUFFER, VK_RESET_FENCES, VK_UNMAP_MEMORY, VK_WAIT_FOR_FENCES};
use super::utils::find_memory_type;
use super::{ALLOC_NOT_SUPPORT, INITS};

use super::VkIcdSurfaceXcb;
use super::SURFACE_CAP;
use super::SURFACE_MODES;

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceProcAddr(
    device: VkDevice,
    pName: *const ::std::os::raw::c_char,
) -> PFN_vkVoidFunction {
    let name_map = LazyLock::force(&super::HIJACKED_FNS);
    if let Some(f) = name_map.get(CStr::from_ptr(pName)) {
        *f
    } else {
        VK_GET_DEVICE_PROC_ADDR.unwrap()(device, pName)
    }
}

struct SwapChainFIFO {
    device: VkDevice,
    image: VkImage,
    image_memory: VkDeviceMemory,
    surface: *const VkIcdSurfaceXcb,

    /// Because VkSemaphore and VkFence are opaque handles, we have to use a queue to signal them.
    queue: VkQueue,

    command_pool: VkCommandPool,
    command_buffer: VkCommandBuffer,
    width: u32,
    height: u32,
    buffer: VkBuffer,
    fence: VkFence,
    buffer_memory: VkDeviceMemory,
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSwapchainKHR(
    device: VkDevice,
    pCreateInfo: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchain: *mut VkSwapchainKHR,
) -> VkResult {
    let mut is_ok = true;

    let mut create_info = pCreateInfo.read();
    if !pAllocator.is_null() {
        println!("{ALLOC_NOT_SUPPORT}");
    }
    if !SURFACE_MODES.contains(&create_info.presentMode) {
        println!("presentMode for swapchain creation not supported, using supported one");
        create_info.presentMode = SURFACE_MODES[0];
    }
    if SURFACE_CAP.supportedCompositeAlpha & create_info.compositeAlpha == 0 {
        println!("compositeAlpha for swapchain creation not supported, using supported one");
        create_info.compositeAlpha = SURFACE_CAP.supportedCompositeAlpha;
    }
    if SURFACE_CAP.supportedTransforms & create_info.preTransform == 0 {
        println!("preTransform is currently not supported.");
    }
    if SURFACE_CAP.maxImageArrayLayers < create_info.imageArrayLayers {
        println!("maxImageArrayLayers for swapchain creation not supported, using supported one");
        create_info.imageArrayLayers = SURFACE_CAP.maxImageArrayLayers;
    }

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

    let buffer_size = width as u64 * height as u64 * 4;
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

    let swap_chain = SwapChainFIFO {
        device,
        image,
        image_memory,
        surface: create_info.surface as *const VkIcdSurfaceXcb,
        queue,
        command_pool,
        command_buffer,
        width,
        height,
        buffer,
        fence,
        buffer_memory,
    };
    let boxed_swap_chain = Box::new(swap_chain);
    *pSwapchain = Box::into_raw(boxed_swap_chain) as _;

    is_ok as i32 - 1
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySwapchainKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    if !pAllocator.is_null() {
        println!("{ALLOC_NOT_SUPPORT}");
    }

    let swap_chain = Box::from_raw(swapchain as *mut SwapChainFIFO);
    VK_DESTROY_IMAGE.unwrap()(device, swap_chain.image, null_mut());
    VK_DESTROY_COMMAND_POOL.unwrap()(device, swap_chain.command_pool, null_mut());
    VK_DESTROY_FENCE.unwrap()(device, swap_chain.fence, null_mut());
    VK_DESTROY_BUFFER.unwrap()(device, swap_chain.buffer, null_mut());
    VK_FREE_MEMORY.unwrap()(device, swap_chain.buffer_memory, null_mut());
    VK_FREE_MEMORY.unwrap()(device, swap_chain.image_memory, null_mut());

    drop(swap_chain)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSwapchainImagesKHR(
    _device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainImageCount: *mut u32,
    pSwapchainImages: *mut VkImage,
) -> VkResult {
    let swapchain = &*(swapchain as *const SwapChainFIFO);
    if pSwapchainImages.is_null() {
        *pSwapchainImageCount = 1;
        VkResult_VK_SUCCESS
    } else {
        if *pSwapchainImageCount < 1 {
            VkResult_VK_INCOMPLETE
        } else {
            *pSwapchainImages = swapchain.image;
            VkResult_VK_SUCCESS
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireNextImageKHR(
    _device: VkDevice,
    swapchain: VkSwapchainKHR,
    _timeout: u64,
    mut semaphore: VkSemaphore,
    fence: VkFence,
    pImageIndex: *mut u32,
) -> VkResult {
    let swapchain = &*(swapchain as *const SwapChainFIFO);

    let submit_info = VkSubmitInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_SUBMIT_INFO,
        pNext: null_mut(),
        waitSemaphoreCount: 0,
        pWaitSemaphores: [].as_ptr(),
        pWaitDstStageMask: [].as_ptr(),
        signalSemaphoreCount: 1,
        pSignalSemaphores: &mut semaphore,
        commandBufferCount: 0,
        pCommandBuffers: [].as_ptr(),
    };

    let result = VK_QUEUE_SUBMIT.unwrap()(swapchain.queue, 1, &submit_info, fence);

    *pImageIndex = 0;
    result
}

#[no_mangle]
pub unsafe extern "C" fn vkQueuePresentKHR(
    queue: VkQueue,
    pPresentInfo: *const VkPresentInfoKHR,
) -> VkResult {
    let info = &*pPresentInfo;

    let count = info.swapchainCount as usize;
    let swapchains = slice::from_raw_parts(info.pSwapchains, count);
    let swapchains: &[&SwapChainFIFO] = transmute(swapchains);
    let mut results = if info.pResults.is_null() {
        None
    } else {
        Some(slice::from_raw_parts_mut(info.pResults, count))
    };

    let stage = VkPipelineStageFlagBits_VK_PIPELINE_STAGE_TRANSFER_BIT;
    let stages = vec![stage; info.waitSemaphoreCount as usize];

    let mut all_is_ok = true;

    for i in 0..count {
        let swapchain = swapchains[i];
        let mut is_ok = true;
        // 1. Copy image to buffer.
        is_ok &= VK_RESET_COMMAND_BUFFER.unwrap()(swapchain.command_buffer, 0) == 0;

        let begin_info = VkCommandBufferBeginInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: null_mut(),
            flags: 0,
            pInheritanceInfo: null_mut(),
        };
        is_ok &= VK_BEGIN_COMMAND_BUFFER.unwrap()(swapchain.command_buffer, &begin_info) == 0;

        let barrier = VkImageMemoryBarrier {
            sType: VkStructureType_VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
            pNext: null_mut(),
            srcAccessMask: 0,
            dstAccessMask: VkAccessFlagBits_VK_ACCESS_TRANSFER_READ_BIT,
            oldLayout: VkImageLayout_VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
            newLayout: VkImageLayout_VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            srcQueueFamilyIndex: INITS.present_queue_family,
            dstQueueFamilyIndex: INITS.present_queue_family,
            image: swapchain.image,
            subresourceRange: VkImageSubresourceRange {
                aspectMask: VkImageAspectFlagBits_VK_IMAGE_ASPECT_COLOR_BIT,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            },
        };
        VK_CMD_PIPELINE_BARRIER.unwrap()(
            swapchain.command_buffer,
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
                width: swapchain.width,
                height: swapchain.height,
                depth: 1,
            },
        };
        VK_CMD_COPY_IMAGE_TO_BUFFER.unwrap()(
            swapchain.command_buffer,
            swapchain.image,
            VkImageLayout_VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            swapchain.buffer,
            1,
            &copy_region,
        );

        is_ok &= VK_END_COMMAND_BUFFER.unwrap()(swapchain.command_buffer) == 0;

        let submit_info = VkSubmitInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: null_mut(),
            waitSemaphoreCount: info.waitSemaphoreCount,
            pWaitSemaphores: info.pWaitSemaphores,
            pWaitDstStageMask: stages.as_ptr(),
            signalSemaphoreCount: 0,
            pSignalSemaphores: [].as_ptr(),
            commandBufferCount: 1,
            pCommandBuffers: &swapchain.command_buffer,
        };
        is_ok &= VK_QUEUE_SUBMIT.unwrap()(queue, 1, &submit_info, swapchain.fence) == 0;

        is_ok &=
            VK_WAIT_FOR_FENCES.unwrap()(swapchain.device, 1, &swapchain.fence, 1, u64::MAX) == 0;
        is_ok &= VK_RESET_FENCES.unwrap()(swapchain.device, 1, &swapchain.fence) == 0;

        // 2. Map buffer and draw the content.
        let size = swapchain.width * swapchain.height * 4;
        let mut data = null_mut();
        is_ok &= VK_MAP_MEMORY.unwrap()(
            swapchain.device,
            swapchain.buffer_memory,
            0,
            size as u64,
            0,
            &mut data,
        ) == 0;

        let surface = &*swapchain.surface;
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
            width: swapchain.width as u16,
            height: swapchain.height as u16,
            dst_x: 0,
            dst_y: 0,
            left_pad: 0,
            depth: 24,
            data: slice::from_raw_parts(data as *mut u8, size as usize),
        });
        is_ok &= connection.check_request(cookie).is_ok();

        is_ok &= connection.flush().is_ok();
        let cookie = connection.send_request_checked(&x::FreeGc { gc });
        is_ok &= connection.check_request(cookie).is_ok();

        connection.into_raw_conn(); // Important, dropping the connection will disconnect the connection.

        VK_UNMAP_MEMORY.unwrap()(swapchain.device, swapchain.buffer_memory);

        all_is_ok &= is_ok;

        if let Some(results) = &mut results {
            results[i] = is_ok as i32 - 1;
        }
    }

    all_is_ok as i32 - 1
}
