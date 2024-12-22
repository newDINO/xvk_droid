use std::{ffi::CStr, ptr::null_mut, sync::LazyLock};

use vulkan_core::{PFN_vkVoidFunction, VkAllocationCallbacks, VkExtensionProperties, VkInstance, VkInstanceCreateInfo, VkResult, VkResult_VK_INCOMPLETE, VkResult_VK_SUCCESS};

use crate::pfn::{VK_CREATE_INSTANCE, VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES, VK_GET_INSTANCE_PROC_ADDR};

use super::XCB_EXT_NAME;

#[no_mangle]
pub unsafe extern "C" fn vkGetInstanceProcAddr(
    instance: VkInstance,
    pName: *const ::std::os::raw::c_char,
) -> PFN_vkVoidFunction {
    let name_map = LazyLock::force(&super::HIJACKED_FNS);
    
    let name = CStr::from_ptr(pName);
    
    if let Some(f) = name_map.get(name) {
        *f
    } else {
        VK_GET_INSTANCE_PROC_ADDR.unwrap()(instance, pName)
    }
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceExtensionProperties(
    pLayerName: *const ::std::os::raw::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut VkExtensionProperties,
) -> VkResult {
    let f = VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES.unwrap();

    if pProperties.is_null() {
        let result = f(pLayerName, pPropertyCount, pProperties);
        *pPropertyCount += 1;
        result
    } else {
        let count = *pPropertyCount;
        let result = f(pLayerName, pPropertyCount, pProperties);

        let mut original_n = 0;
        f(pLayerName, &mut original_n, null_mut());
        *pPropertyCount = original_n + 1;

        if result == VkResult_VK_INCOMPLETE {
            return result;
        }

        if original_n < count {
            let vk_extension_properties = VkExtensionProperties {
                extensionName: [0; 256],
                specVersion: pProperties.read().specVersion,
            };
            let mut ext = vk_extension_properties;
            let name_bytes = XCB_EXT_NAME.to_bytes_with_nul();
            ext.extensionName[..name_bytes.len()].copy_from_slice(name_bytes);
            pProperties.offset(original_n as isize).write(ext);

            VkResult_VK_SUCCESS
        } else {
            VkResult_VK_INCOMPLETE
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateInstance(
    pCreateInfo: *const VkInstanceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pInstance: *mut VkInstance,
) -> VkResult {
    let f = VK_CREATE_INSTANCE.unwrap();

    let count = (&*pCreateInfo).enabledExtensionCount;
    let exts = (&*pCreateInfo).ppEnabledExtensionNames;
    let mut new_exts = Vec::with_capacity(count as usize);

    for i in 0..count {
        let name = CStr::from_ptr(exts.offset(i as isize).read());
        if name == XCB_EXT_NAME {
            continue;
        }
        new_exts.push(exts.offset(i as isize).read());
    }

    let mut new_info = pCreateInfo.read();
    new_info.enabledExtensionCount = new_exts.len() as u32;
    new_info.ppEnabledExtensionNames = new_exts.as_ptr();

    f(&new_info, pAllocator, pInstance)
}