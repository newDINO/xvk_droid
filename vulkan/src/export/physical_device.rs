use std::{ffi::c_void, ptr::null_mut};

use vulkan_core::{VkBool32, VkColorSpaceKHR_VK_COLORSPACE_SRGB_NONLINEAR_KHR, VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR, VkExtent2D, VkFormat_VK_FORMAT_B8G8R8A8_SRGB, VkPhysicalDevice, VkPhysicalDeviceSurfaceInfo2KHR, VkPresentModeKHR, VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR, VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR, VkResult, VkResult_VK_INCOMPLETE, VkResult_VK_SUCCESS, VkStructureType_VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR, VkStructureType_VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR, VkStructureType_VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT, VkStructureType_VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT, VkSurfaceCapabilities2KHR, VkSurfaceCapabilitiesKHR, VkSurfaceFormat2KHR, VkSurfaceFormatKHR, VkSurfaceKHR, VkSurfacePresentModeCompatibilityEXT, VkSurfacePresentModeEXT, VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR};

use super::{check_surface_support, VkIcdSurfaceBase, INITS, VK_ICD_WSI_PLATFORM_XCB};

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceSupportKHR(
    _physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    surface: VkSurfaceKHR,
    pSupported: *mut VkBool32,
) -> VkResult {
    let platform = (surface as *const VkIcdSurfaceBase)
        .read()
        .platform;
    let is_xcb_surface = platform == VK_ICD_WSI_PLATFORM_XCB;
    let is_present_queue = queueFamilyIndex == INITS.present_queue_family;

    *pSupported = (is_present_queue && is_xcb_surface) as u32;
    VkResult_VK_SUCCESS
}

pub const SURFACE_CAP: VkSurfaceCapabilitiesKHR = VkSurfaceCapabilitiesKHR {
    minImageCount: 1,
    maxImageCount: 3,
    currentExtent: VkExtent2D {
        width: 0xFFFFFFFF,
        height: 0xFFFFFFFF,
    },
    maxImageExtent: VkExtent2D {
        width: 4096,
        height: 2160,
    },
    minImageExtent: VkExtent2D {
        width: 1,
        height: 1,
    },
    maxImageArrayLayers: 1,
    supportedTransforms: VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
    supportedCompositeAlpha: VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
    currentTransform: VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
    supportedUsageFlags: u32::MAX,
};
const SURFACE_FORMATS: [VkSurfaceFormatKHR; 1] = [VkSurfaceFormatKHR {
    format: VkFormat_VK_FORMAT_B8G8R8A8_SRGB,
    colorSpace: VkColorSpaceKHR_VK_COLORSPACE_SRGB_NONLINEAR_KHR,
}];
pub const SURFACE_MODES: [u32; 2] = [
    VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR,
    VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR
];
const SURFAFE_PREMENT_MODE_CAPABILIEIS: VkSurfacePresentModeCompatibilityEXT = VkSurfacePresentModeCompatibilityEXT {
    sType: VkStructureType_VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT,
    pNext: null_mut(),
    presentModeCount: SURFACE_MODES.len() as u32,
    pPresentModes: SURFACE_MODES.as_ptr() as *mut u32,
};

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    _physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult {
    check_surface_support(surface);
    *pSurfaceCapabilities = SURFACE_CAP;
    VkResult_VK_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceFormatsKHR(
    _physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormatKHR,
) -> VkResult {
    check_surface_support(surface);
    *pSurfaceFormatCount = SURFACE_FORMATS.len() as u32;
    if pSurfaceFormats.is_null() {
        VkResult_VK_SUCCESS
    } else {
        let given_count = *pSurfaceFormatCount as usize;
        let max_count = given_count.min(SURFACE_FORMATS.len());
        let formats = std::slice::from_raw_parts_mut(pSurfaceFormats, max_count);
        formats.copy_from_slice(&SURFACE_FORMATS[..max_count]);
        if given_count < SURFACE_FORMATS.len() {
            VkResult_VK_INCOMPLETE
        } else {
            VkResult_VK_SUCCESS
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfacePresentModesKHR(
    _physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut VkPresentModeKHR,
) -> VkResult {
    check_surface_support(surface);
    *pPresentModeCount = SURFACE_MODES.len() as u32;
    if pPresentModes.is_null() {
        VkResult_VK_SUCCESS
    } else {
        let given_count = *pPresentModeCount as usize;
        let count = given_count.min(SURFACE_MODES.len());
        let modes = std::slice::from_raw_parts_mut(pPresentModes, count);
        modes.copy_from_slice(&SURFACE_MODES[..count]);
        if given_count < SURFACE_MODES.len() {
            VkResult_VK_INCOMPLETE
        } else {
            VkResult_VK_SUCCESS
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(
    _physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR,
) -> VkResult {
    let info = &*pSurfaceInfo;
    let p_next = if !info.pNext.is_null() {
        let p_next = &*(info.pNext as *const VkSurfacePresentModeEXT);
        if p_next.sType == VkStructureType_VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT {
            &SURFAFE_PREMENT_MODE_CAPABILIEIS as *const _ as *mut c_void
        } else {
            null_mut()
        }
    } else {
        null_mut()
    };

    *pSurfaceCapabilities = VkSurfaceCapabilities2KHR {
        sType: VkStructureType_VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR,
        pNext: p_next,
        surfaceCapabilities: SURFACE_CAP
    };
    VkResult_VK_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceFormats2KHR(
    _physicalDevice: VkPhysicalDevice,
    _pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormat2KHR,
) -> VkResult {
    *pSurfaceFormatCount = SURFACE_FORMATS.len() as u32;
    if pSurfaceFormats.is_null() {
        VkResult_VK_SUCCESS
    } else {
        let given_count = *pSurfaceFormatCount as usize;
        let max_count = given_count.min(SURFACE_FORMATS.len());
        let formats = std::slice::from_raw_parts_mut(pSurfaceFormats, max_count);
        for i in 0..formats.len() {
            formats[i].sType = VkStructureType_VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR;
            formats[i].pNext = null_mut();
            formats[i].surfaceFormat = SURFACE_FORMATS[i];
        }
        if given_count < SURFACE_FORMATS.len() {
            VkResult_VK_INCOMPLETE
        } else {
            VkResult_VK_SUCCESS
        }
    }
}