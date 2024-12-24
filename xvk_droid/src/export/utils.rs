use vulkan_core::VkSurfaceKHR;

use super::{VkIcdSurfaceBase, INITS, VK_ICD_WSI_PLATFORM_XCB};

pub unsafe fn find_memory_type(supported_bit: u32, required_type: u32) -> Option<u32> {
    for i in 0..INITS.memory_properties.memoryTypeCount {
        let supported = (supported_bit & (1 << i)) != 0;
        let needed = (INITS.memory_properties.memoryTypes[i as usize].propertyFlags & required_type) != 0;
        if supported && needed {
            return Some(i)
        }
    }
    None
}

pub unsafe fn check_surface_support(
    surface: VkSurfaceKHR,
) {
    if !surface.is_null() {
        let platform = (surface as *const VkIcdSurfaceBase)
            .read()
            .platform;
        if platform != VK_ICD_WSI_PLATFORM_XCB {
            println!("Warn: surface is not an xcb surface");
        }
    }
}