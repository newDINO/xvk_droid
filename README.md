## XVK_DROID: Add X11 window surface support for Android Vulkan driver in Termux.

### Project stage

Proved viable (Successfully tested on simple programs).

### Tests
#### Programs
- [x] xcb hello triangle
- [x] GLFW hello trangle
- [x] vkcube
- [] vkmark (Require mailbox swapchain support)
- [] box64 wine dxvk
- [] zink
- [] wgpu programs

### TODO
- [] Support untested programs in the list above.
- [] Provide prebuilt binaries and make it a deb package.
- [] Publish to Termux repo.
- [] Make it an ICD driver to support coexisting with other vulkan drivers.
- [] Reduce overhead to improve performance.

### How to use the program

#### Build the program
Requirement: rust

Run in the repository:  
```
cargo build -r -p vulkan
```  
The resulting library will be `target/release/libvulkan.so`

#### Use the program to replace the default libvulkan

Run in the repository if you have make installed:  
```
make create_link
```
Or run
```
ln -s target/release/libvulkan.so libvulkan.so
ln -s target/release/libvulkan.so libvulkan.so.1
```
if you don't.  

This will create symbolic links to the library in the root of the repository, the reason to do this is  
1. for convenience  
2. GLFW load `libvulkan.so.1` rather than libvulkan.so. We need to create an additional `libvulkan.so.1` for programs using GLFW to work.

To run program using the `libvulkan.so` provided by this project. Run  
```
LD_LIBRARY_PATH=$PWD:$LD_LIBRARY_PATH program_name
```  
in the root of the repository.  
For example:  
```
LD_LIBRARY_PATH=$PWD:$LD_LIBRARY_PATH vkcube
```  
vkcube can be installed using  
```
pkg install vulkan-tools
```

Or you can export envirenment variables to use this library to run all the program in the this shell.  
```
export LD_LIBRARY_PATH=$PWD:$LD_LIBRARY_PATH
```  
For example:
```
export LD_LIBRARY_PATH=$PWD:$LD_LIBRARY_PATH
vkcube
```