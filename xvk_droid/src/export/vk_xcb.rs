use std::ffi::c_void;

use vulkan_core::{
    VkAllocationCallbacks, VkBool32, VkFlags,VkInstance, VkPhysicalDevice, VkResult,
    VkResult_VK_SUCCESS, VkStructureType, VkSurfaceKHR,
};
use xcb::{ffi::xcb_connection_t, x::Window};

use super::{ALLOC_NOT_SUPPORT, INITS};

pub const VK_ICD_WSI_PLATFORM_XCB: VkIcdWsiPlatform = 3;
pub type VkIcdWsiPlatform = ::std::os::raw::c_uint;

#[repr(C)]
pub struct VkIcdSurfaceBase {
    pub platform: VkIcdWsiPlatform,
}

#[repr(C)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFlags,
    pub connection: *mut xcb_connection_t,
    pub window: Window,
}

#[repr(C)]
pub struct VkIcdSurfaceXcb {
    pub base: VkIcdSurfaceBase,
    pub connection: *mut xcb_connection_t,
    pub window: Window,
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateXcbSurfaceKHR(
    _instance: VkInstance,
    pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    surface: *mut VkSurfaceKHR,
) -> VkResult {
    if !pAllocator.is_null() {
        println!("{ALLOC_NOT_SUPPORT}");
    }
    let boxed_surface = Box::new(VkIcdSurfaceXcb {
        base: VkIcdSurfaceBase {
            platform: VK_ICD_WSI_PLATFORM_XCB,
        },
        connection: pCreateInfo.read().connection,
        window: pCreateInfo.read().window,
    });
    *surface = Box::into_raw(boxed_surface) as VkSurfaceKHR;
    VkResult_VK_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySurfaceKHR(
    _instance: VkInstance,
    surface: VkSurfaceKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    if !pAllocator.is_null() {
        println!("{ALLOC_NOT_SUPPORT}");
    }
    let boxed_surface = Box::from_raw(surface as *mut VkIcdSurfaceXcb);
    drop(boxed_surface);
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceXcbPresentationSupportKHR(
    _physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    _connection: *const xcb_connection_t,
    _visual_id: u32,
) -> VkBool32 {
    (INITS.present_queue_family == queueFamilyIndex) as u32
}
