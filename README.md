## XVK_DROID: Add X11 window surface support for Android Vulkan driver in Termux.

### Project stage

Proved viable (Successfully tested on simple programs).  
Current implementation first copies VkImage inside the swapchain to a VkBuffer, then uses xcb PutImage request to copies the image data inside the VkBuffer to the window surface.

### Tests
#### Programs
✅: Works fine.  
🛠️: Tested, but not functioning well.  
❌: Tested, but not workding or barely working.  
⏹️: Untested.  

- ✅ xcb hello triangle
- ✅ GLFW hello triangle
- ✅ vkcube (Currently only support BGRA_SRGB surface format, not prefered by vkcube)
- ✅ vkmark (~900 FPS on SD8Gen3)
- 🛠️ gears (Only shows the red gear's first face)
- 🛠️ glmark2 (Can't finish the benchmark, crash in the middle of it, error: `ralloc_header *get_header(const void *): assertion "info->canary == CANARY" failed`. ~350-400 FPS on SD8Gen3)
- ❌ firefox (Only 4 FPS in WebGL Samples Aquarium)
- ❌ chromium (When enabling vulkan flag: in chome://gpu, vulkan info shows it is using xvk_droid, but dawn is using the original vulkan loader)
- ❌ programs inside proot-distro (When compiling using glibc, the program can't load android dynamic library, we need to compile bionic libdl to work)
- ❌ glibc packages (Same reason as proot-distro programs)
- ⏹️ box64 wine dxvk (need to fix glibc packages first)
- ⏹️ Minecraft (Require compiling OpenJFX for termux)
- ⏹️ wgpu programs (winit is assuming termux an android environment and not using x11 surface)

### TODO
- Support test programs listed above.
- Support loading android `libvulkan.so` when compiled using glibc, so that it can be used for glibc-repo programs and proot\chroot gnu linux programs.
- Provide prebuilt binaries and make it a deb package.
- Publish to Termux repo.
- Make it an ICD driver to support coexisting with other vulkan drivers.
- Reduce overhead to improve performance.

### How to use the program

#### Build the program
Requirement: `rust`

Run in the repository:  
```
cargo build -r -p xvk_droid
```  
The resulting library will be `target/release/libxvk_droid.so`

#### Use the program to replace the default libvulkan

Run in the repository:  
```
make create_link
```
if you have `make` installed, or run
```
ln -s target/release/libxvk_droid.so libvulkan.so
ln -s target/release/libxvk_droid.so libvulkan.so.1
```
if you don't.  

This will create symbolic links to the library in the root of the repository, the reason to do this is  
1. for convenience  
2. GLFW load `libvulkan.so.1` rather than libvulkan.so. We need to create an additional `libvulkan.so.1` for programs using GLFW to work.

To run program using the xvk_droid. Run  
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
pkg i vulkan-tools
```

Or you can export envirenment variables to use xvk_droid to run all the program in the this shell.  
```
export LD_LIBRARY_PATH=$PWD:$LD_LIBRARY_PATH
```  
Or using
```
. set_env.sh
```
for convenience. Afterwards the programs ran in the same shell will use xvk_droid as the vulkan implementation.
For example:
```
. set_env.sh
vkmark
```
vkmark is in the tur-repo, to install it, run:
```
pkg i tur-repo
pkg i vkmark
```