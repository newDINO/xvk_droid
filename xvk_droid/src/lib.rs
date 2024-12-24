use std::ffi::CStr;

use dlfcn::{dlopen, RTLD_NOW};
mod dlfcn;

// All original function pointers.
mod pfn;
// Initiate original functions pointers using dlsym.
mod init;

// Swapchain implementations.
mod swapchain;

// All exposed vulkan functions.
mod export;

// The original vulkan implementation to hijack.
const SO_PATH: &CStr = c"/system/lib64/libvulkan.so";
// const SO_PATH: &CStr = c"/data/data/com.termux/files/usr/lib/libvulkan.so";
// const SO_PATH: &CStr = c"/data/data/com.termux/files/usr/lib/libvk_swiftshader.so";

// Initiations
#[ctor::ctor]
unsafe fn init_xvk() {
    println!("XVK_DROID loaded");
    let handle = dlopen(SO_PATH.as_ptr(), RTLD_NOW as i32);
    // let get_pfn = dlsym(handle, c"vkGetInstanceProcAddr".as_ptr());
    init::init(handle);
    export::init();
}
