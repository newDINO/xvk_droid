// pub const RTLD_LOCAL: u32 = 0;
// pub const RTLD_LAZY: u32 = 1;
pub const RTLD_NOW: u32 = 2;
// pub const RTLD_NOLOAD: u32 = 4;
// pub const RTLD_GLOBAL: u32 = 256;
// pub const RTLD_NODELETE: u32 = 4096;

unsafe extern "C" {
    #[doc = " [dlopen(3)](http://man7.org/linux/man-pages/man3/dlopen.3.html)\n loads the given shared library.\n\n Returns a pointer to an opaque handle for use with other <dlfcn.h> functions\n on success, and returns NULL on failure, in which case dlerror() can be used\n to retrieve the specific error."]
    pub fn dlopen(
        __filename: *const ::std::os::raw::c_char,
        __flag: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    #[doc = " [dlsym(3)](http://man7.org/linux/man-pages/man3/dlsym.3.html)\n returns a pointer to the symbol with the given name in the shared\n library represented by the given handle. The handle may have been\n returned from dlopen(), or can be RTLD_DEFAULT or RTLD_NEXT.\n\n Returns the address of the symbol on success, and returns NULL on failure,\n in which case dlerror() can be used to retrieve the specific error."]
    pub fn dlsym(
        __handle: *mut ::std::os::raw::c_void,
        __symbol: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    #[doc = " [dlclose(3)](http://man7.org/linux/man-pages/man3/dlclose.3.html)\n decrements the reference count for the given shared library (and\n any libraries brought in by that library's DT_NEEDED entries).\n\n If a library's reference count hits zero, it may be unloaded.\n Code that relies on this is not portable, and may not work on\n future versions of Android.\n\n dlclose() is dangerous because function pointers may or may not\n be rendered invalid, global data may or may not be rendered invalid,\n and memory may or may not leak. Code with global constructors is\n especially problematic. Instead of dlclose, prefer to leave the\n library open or, if cleanup is necessary, dlopen() the library in\n a child process which can later be killed by the parent or call\n exit() itself.\n\n Note also that dlclose() interacts badly with thread local variables\n with non-trivial destructors, with the\n (exact behavior varying by API level)[https://android.googlesource.com/platform/bionic/+/main/android-changes-for-ndk-developers.md#dlclose-interacts-badly-with-thread-local-variables-with-non_trivial-destructors].\n\n Returns 0 on success, and returns -1 on failure, in which case\n dlerror() can be used to retrieve the specific error."]
    pub fn dlclose(__handle: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
// unsafe extern "C" {
//     #[doc = " [dlerror(3)](http://man7.org/linux/man-pages/man3/dlerror.3.html)\n returns a human-readable error message describing the most recent\n failure from one of the <dlfcn.h> functions on the calling thread.\n\n This function also clears the error, so a second call (or a call\n before any failure) will return NULL.\n\n Returns a pointer to an error on success, and returns NULL if no\n error is pending."]
//     pub fn dlerror() -> *mut ::std::os::raw::c_char;
// }
