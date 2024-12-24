notes:
1. Using depth 24 for xcb_put_image, which I don't know the meaning.
2. What is VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR?
3. If the presentation thread can't finish present continously and need to wait for the main thread during some process, than when rendering framerate is low, presenting framerate could be even lower.
4. Try blocking Android surface?

Unimplemented:
1. VK_KHR_get_surface_capabilities2
2. VK_EXT_surface_maintenance1