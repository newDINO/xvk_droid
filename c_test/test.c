#include <dlfcn.h>

int main() {
    dlopen("/data/data/com.termux/files/home/tests/xvk_droid/target/release/libvulkan.so", RTLD_NOW);
}
