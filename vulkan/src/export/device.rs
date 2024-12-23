use core::slice;
use std::mem::transmute;
use std::{ffi::CStr, sync::LazyLock};

use vulkan_core::*;

use crate::pfn::*;
use crate::swapchain::SwapChain;
use super::ALLOC_NOT_SUPPORT;

use super::SURFACE_CAP;

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

    let swapchain = SwapChain::new(device, &create_info, &mut is_ok);

    let boxed_swapchain = Box::new(swapchain);
    *pSwapchain = Box::into_raw(boxed_swapchain) as _;

    is_ok as i32 - 1
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySwapchainKHR(
    _device: VkDevice,
    swapchain: VkSwapchainKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    if !pAllocator.is_null() {
        println!("{ALLOC_NOT_SUPPORT}");
    }

    let boxed_swapchain = Box::from_raw(swapchain as *mut SwapChain);
    drop(boxed_swapchain)
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSwapchainImagesKHR(
    _device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainImageCount: *mut u32,
    pSwapchainImages: *mut VkImage,
) -> VkResult {
    let swapchain = &*(swapchain as *const SwapChain);
    swapchain.get_swapchain_images(pSwapchainImageCount, pSwapchainImages)
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireNextImageKHR(
    _device: VkDevice,
    swapchain: VkSwapchainKHR,
    _timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    pImageIndex: *mut u32,
) -> VkResult {
    let swapchain = &*(swapchain as *const SwapChain);
    swapchain.acquire_next_image(semaphore, fence, pImageIndex)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueuePresentKHR(
    queue: VkQueue,
    pPresentInfo: *const VkPresentInfoKHR,
) -> VkResult {
    let info = &*pPresentInfo;

    let count = info.swapchainCount as usize;
    let swapchains = slice::from_raw_parts(info.pSwapchains, count);
    let swapchains: &[&SwapChain] = transmute(swapchains);
    let mut results = if info.pResults.is_null() {
        None
    } else {
        Some(slice::from_raw_parts_mut(info.pResults, count))
    };
    let image_indices = slice::from_raw_parts(info.pImageIndices, count);
    let mut all_is_ok = true;

    for i in 0..count {
        let is_ok = swapchains[i].queue_present(
            queue,
            image_indices[i],
            info.waitSemaphoreCount,
            info.pWaitSemaphores,
        );

        all_is_ok &= is_ok;

        if let Some(results) = &mut results {
            results[i] = is_ok as i32 - 1;
        }
    }

    all_is_ok as i32 - 1
}
