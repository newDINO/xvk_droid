use std::ffi::CStr;

use dlfcn::{dlopen, RTLD_NOW};

mod dlfcn;

mod init;
mod pfn;

mod export;

const SO_PATH: &CStr = c"/system/lib64/libvulkan.so";
// const SO_PATH: &CStr = c"/data/data/com.termux/files/usr/lib/libvulkan.so";
// const SO_PATH: &CStr = c"/data/data/com.termux/files/usr/lib/libvk_swiftshader.so";

// Initiations
#[no_mangle]
pub unsafe extern "C" fn _init() {
    println!("XVK_DROID loaded");
    let handle = dlopen(SO_PATH.as_ptr(), RTLD_NOW as i32);
    // let get_pfn = dlsym(handle, c"vkGetInstanceProcAddr".as_ptr());
    init::init(handle);
    export::init();
}
