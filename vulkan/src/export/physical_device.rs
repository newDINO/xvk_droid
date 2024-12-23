use vulkan_core::{VkBool32, VkColorSpaceKHR_VK_COLORSPACE_SRGB_NONLINEAR_KHR, VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR, VkExtent2D, VkFormat_VK_FORMAT_B8G8R8A8_SRGB, VkPhysicalDevice, VkPresentModeKHR, VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR, VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR, VkResult, VkResult_VK_INCOMPLETE, VkResult_VK_SUCCESS, VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR, VkSurfaceKHR, VkSurfaceTransformFlagBitsKHR_VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR};

use super::{VkIcdSurfaceBase, INITS, VK_ICD_WSI_PLATFORM_XCB};

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

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    _physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult {
    if !surface.is_null() {
        let platform = (surface as *const VkIcdSurfaceBase)
            .read()
            .platform;
        if platform != VK_ICD_WSI_PLATFORM_XCB {
            println!("Warn: surface is not an xcb surface");
        }
    }
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
    if !surface.is_null() {
        let platform = (surface as *const VkIcdSurfaceBase)
            .read()
            .platform;
        if platform != VK_ICD_WSI_PLATFORM_XCB {
            println!("Warn: surface is not an xcb surface");
        }
    }
    if pSurfaceFormats.is_null() {
        *pSurfaceFormatCount = SURFACE_FORMATS.len() as u32;
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
    if !surface.is_null() {
        let platform = (surface as *const VkIcdSurfaceBase)
            .read()
            .platform;
        if platform != VK_ICD_WSI_PLATFORM_XCB {
            println!("Warn: surface is not an xcb surface");
        }
    }
    if pPresentModes.is_null() {
        *pPresentModeCount = SURFACE_MODES.len() as u32;
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