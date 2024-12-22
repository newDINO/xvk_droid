use std::fs;
use std::fmt::Write;

// const ADDED_FNS: [&str; 2] = [
//     "vkCreateXcbSurfaceKHR",
//     "vkGetPhysicalDeviceXcbPresentationSupportKHR"
// ];

const WRITEN_FNS: [&str; 16] = [
    "vkCreateXcbSurfaceKHR",
    "vkDestroySurfaceKHR",
    "vkGetPhysicalDeviceXcbPresentationSupportKHR",
    
    "vkGetInstanceProcAddr",
    "vkEnumerateInstanceExtensionProperties",
    "vkCreateInstance",

    "vkGetPhysicalDeviceSurfaceSupportKHR",
    "vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
    "vkGetPhysicalDeviceSurfaceFormatsKHR",
    "vkGetPhysicalDeviceSurfacePresentModesKHR",

    "vkGetDeviceProcAddr",
    "vkCreateSwapchainKHR",
    "vkGetSwapchainImagesKHR",
    "vkAcquireNextImageKHR",
    "vkQueuePresentKHR",
    "vkDestroySwapchainKHR",
];

const NON_VK_FNS: [&str; 3] = [
    "android_get_application_target_sdk_version",
    "__system_property_get",
    "atoi",
];

const INIT_HEAD: &str = 
"use std::{ffi::c_void, mem::transmute};
use crate::dlfcn::dlsym;
use crate::pfn::*;

pub unsafe fn init(handle: *mut c_void) {
";
const INIT_TAIL: &str = "}";

const INVARIENT_HEAD: &str = 
"use crate::pfn::*;
use vulkan_core::*;\n\n";

fn main() {
    let header = fs::read_to_string("vulkan_core/src/lib.rs").unwrap();

    let mut in_function = false;

    let mut pfn = "use vulkan_core::*;\n\n".to_owned();
    let mut init = INIT_HEAD.to_owned();
    let mut invarients = INVARIENT_HEAD.to_owned();

    let mut upper_snake = String::new();
    let mut param_and_return = String::new();
    let mut not_vk_fn = false;
    let mut hijacked = false;
    for line in header.split('\n') {
        if line.starts_with("unsafe extern \"C\" {") {
            in_function = true;
        } else if line.starts_with('}') {
            in_function = false;
        } else if in_function {
            if let Some(end) = line.strip_prefix("    pub fn ") {
                let (name, after_name) = end.split_once('(').unwrap();
                param_and_return.clear();
                param_and_return.push_str(after_name);

                not_vk_fn = false;
                if NON_VK_FNS.contains(&name) {
                    not_vk_fn = true;
                    continue;
                }
                if WRITEN_FNS.contains(&name) {
                    hijacked = true;
                } else {
                    hijacked = false;
                }

                let snake = camel_to_snake(name);
                upper_snake = snake.to_ascii_uppercase();

                write!(pfn, "pub static mut {}: PFN_{} = None;\n", upper_snake, name).unwrap();

                // if name == "vkGetInstanceProcAddr" {
                //     write!(init, "    {} = transmute(get_pfn);\n", upper_snake).unwrap();
                // } else {
                //     write!(init, "    {} = transmute(get_pfn(null_mut(), c\"{}\".as_ptr()));\n", upper_snake, name).unwrap();
                // }
                write!(init, "    {} = transmute(dlsym(handle, c\"{}\".as_ptr()));\n", upper_snake, name).unwrap();

                if !hijacked {
                    write!(invarients, "#[no_mangle]\npub unsafe extern \"C\" fn {}\n", end).unwrap();
                }
            } else {
                if not_vk_fn || hijacked {
                    continue;
                }
                let space_removed = line.strip_prefix("    ").unwrap();
                param_and_return.push_str(space_removed);
                invarients.push_str(space_removed);
                invarients.push('\n');
            }
            if not_vk_fn || hijacked {
                continue;
            }
            if line.ends_with(';') {
                invarients.pop().unwrap();
                invarients.pop().unwrap();
                invarients.push_str(" {\n");
                write!(invarients, "    {}.unwrap()(", upper_snake).unwrap();
                
                let params = param_and_return.split_once(')').unwrap().0;
                for p in params.split(',') {
                    if p.len() == 0 {
                        continue;
                    }
                    let param_name = remove_space(p.split_once(':').unwrap().0);
                    invarients.push_str(&param_name);
                    invarients.push_str(", ");
                }
                invarients.pop(); // ' '
                invarients.pop(); // ','
                invarients.push_str(")\n}\n\n");
            }
        }
    }

    fs::write("vulkan/src/pfn.rs", pfn).unwrap();

    init.push_str(INIT_TAIL);
    fs::write("vulkan/src/init.rs", init).unwrap();

    fs::write("vulkan/src/export/invarient.rs", invarients).unwrap();
}

fn remove_space(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c != ' ' {
            result.push(c);
        }
    }
    result
}

fn camel_to_snake(camel: &str) -> String {
    let mut result = String::with_capacity(camel.len());

    let mut last_upper = false;
    for c in camel.chars() {
        let this_upper = c.is_ascii_uppercase();
        if !last_upper && c.is_ascii_uppercase() {
            result.push('_');
        }
        if this_upper {
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
        last_upper = this_upper;
    }

    result
}