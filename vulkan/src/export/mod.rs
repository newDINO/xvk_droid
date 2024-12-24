#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


use std::{collections::HashMap, ffi::CStr, mem::{transmute, MaybeUninit}, ptr::null_mut, sync::LazyLock};

use vulkan_core::{PFN_vkVoidFunction, VkInstanceCreateInfo, VkMemoryHeap, VkMemoryType, VkPhysicalDeviceMemoryProperties, VkQueueFamilyProperties, VkQueueFlagBits_VK_QUEUE_COMPUTE_BIT, VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT, VkQueueFlagBits_VK_QUEUE_TRANSFER_BIT, VkResult_VK_SUCCESS, VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO};

use crate::pfn::{VK_CREATE_INSTANCE, VK_DESTROY_INSTANCE, VK_ENUMERATE_PHYSICAL_DEVICES, VK_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES, VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES};

mod utils;
pub use utils::*;

// Unchanged functions
mod invarient;
// Hijacked functions
mod vk_xcb;
pub use vk_xcb::*;
mod instance;
use instance::*;
mod physical_device;
use physical_device::*;
mod device;
use device::*;

pub const ALLOC_NOT_SUPPORT: &str = "xcb_droid: Custom allocator currently not supported";
const XCB_EXT_NAME: &CStr = c"VK_KHR_xcb_surface";


// Initiation
static HIJACKED_FNS: LazyLock<HashMap<&CStr, PFN_vkVoidFunction>> = LazyLock::new(|| unsafe {
    HashMap::from([
        // vk_xcb
        (c"vkCreateXcbSurfaceKHR", transmute(vkCreateXcbSurfaceKHR as *const usize)),
        (c"vkDestroySurfaceKHR", transmute(vkDestroySurfaceKHR as *const usize)),
        (c"vkGetPhysicalDeviceXcbPresentationSupportKHR", transmute(vkGetPhysicalDeviceXcbPresentationSupportKHR as *const usize)),
        // instance
        (c"vkGetInstanceProcAddr", transmute(vkGetInstanceProcAddr as *const usize)),
        (c"vkEnumerateInstanceExtensionProperties", transmute(vkEnumerateInstanceExtensionProperties as *const usize)),
        (c"vkCreateInstance", transmute(vkCreateInstance as *const usize)),
        // physical device
        (c"vkGetPhysicalDeviceSurfaceSupportKHR", transmute(vkGetPhysicalDeviceSurfaceSupportKHR as *const usize)),
        (c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR", transmute(vkGetPhysicalDeviceSurfaceCapabilitiesKHR as *const usize)),
        (c"vkGetPhysicalDeviceSurfaceFormatsKHR", transmute(vkGetPhysicalDeviceSurfaceFormatsKHR as *const usize)),
        (c"vkGetPhysicalDeviceSurfacePresentModesKHR", transmute(vkGetPhysicalDeviceSurfacePresentModesKHR as *const usize)),
        (c"vkGetPhysicalDeviceSurfaceCapabilities2KHR", transmute(vkGetPhysicalDeviceSurfaceCapabilities2KHR as *const usize)),
        (c"vkGetPhysicalDeviceSurfaceFormats2KHR", transmute(vkGetPhysicalDeviceSurfaceFormats2KHR as *const usize)),
        // device
        (c"vkGetDeviceProcAddr", transmute(vkGetDeviceProcAddr as *const usize)),
        (c"vkCreateSwapchainKHR", transmute(vkCreateSwapchainKHR as *const usize)),
        (c"vkGetSwapchainImagesKHR", transmute(vkGetSwapchainImagesKHR as *const usize)),
        (c"vkAcquireNextImageKHR", transmute(vkAcquireNextImageKHR as *const usize)),
        (c"vkQueuePresentKHR", transmute(vkQueuePresentKHR as *const usize)),
        (c"vkDestroySwapchainKHR", transmute(vkDestroySwapchainKHR as *const usize)),
    ])
});

pub struct Inits {
    pub present_queue_family: u32,
    pub memory_properties: VkPhysicalDeviceMemoryProperties,
}

pub static mut INITS: Inits = Inits {
    present_queue_family: 0,
    memory_properties: VkPhysicalDeviceMemoryProperties {
        memoryHeapCount: 0,
        memoryHeaps: [VkMemoryHeap { size: 0, flags: 0 }; 16],
        memoryTypeCount: 0,
        memoryTypes: [VkMemoryType { propertyFlags: 0, heapIndex: 0 }; 32]
    }
};

/// Must be after the initiation of imported function pointers.
pub unsafe fn init() {
    let instance_info = VkInstanceCreateInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: null_mut(),
        flags: 0,
        enabledLayerCount: 0,
        ppEnabledLayerNames: [].as_ptr(),
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: [].as_ptr(),
        pApplicationInfo: null_mut(),
    };
    let mut instance = null_mut();
    if VK_CREATE_INSTANCE.unwrap()(&instance_info, null_mut(), &mut instance) != VkResult_VK_SUCCESS {
        println!("XVK_DROID: init instance creation failed");
    }

    let mut physical_device_count = 0;
    VK_ENUMERATE_PHYSICAL_DEVICES.unwrap()(instance, &mut physical_device_count, null_mut());
    let mut physical_devices = vec![null_mut(); physical_device_count as usize];
    VK_ENUMERATE_PHYSICAL_DEVICES.unwrap()(
        instance,
        &mut physical_device_count,
        physical_devices.as_mut_ptr(),
    );

    if physical_device_count > 1 {
        println!("XVK_DROID does not fully support driver with more than one physical device yet");
    }

    let mut memory_properties = MaybeUninit::uninit();
    VK_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES.unwrap()(
        physical_devices[0],
        memory_properties.as_mut_ptr(),
    );

    let mut queue_family_count = 0;

    VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES.unwrap()(
        physical_devices[0],
        &mut queue_family_count,
        null_mut(),
    );
    let properties =
        vec![MaybeUninit::<VkQueueFamilyProperties>::uninit(); queue_family_count as usize];
    let mut properties: Vec<VkQueueFamilyProperties> = transmute(properties);
    VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES.unwrap()(
        physical_devices[0],
        &mut queue_family_count,
        properties.as_mut_ptr(),
    );

    let mut present_queue_family = None;
    for i in 0..properties.len() {
        if properties[i].queueFlags & VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT != 0
            || properties[i].queueFlags & VkQueueFlagBits_VK_QUEUE_TRANSFER_BIT != 0
            || properties[i].queueFlags & VkQueueFlagBits_VK_QUEUE_COMPUTE_BIT != 0
        {
            present_queue_family = Some(i as u32);
            break;
        }
    }
    let present_queue_family = if let Some(family) = present_queue_family {
        family
    } else {
        println!("XVK_DROID: Failed to get a queue family with transfer support, using 0.");
        0
    };

    VK_DESTROY_INSTANCE.unwrap()(instance, null_mut());

    INITS = Inits {
        present_queue_family,
        memory_properties: memory_properties.assume_init()
    }
}
