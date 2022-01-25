#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use super::sdl_bindings::*;
use ash::vk::{Instance, SurfaceKHR};

pub type SDL_vulkanInstance = Instance;
pub type SDL_vulkanSurface = SurfaceKHR;
extern "C" {
    #[doc = " Dynamically load the Vulkan loader library."]
    #[doc = ""]
    #[doc = " This should be called after initializing the video driver, but before"]
    #[doc = " creating any Vulkan windows. If no Vulkan loader library is loaded, the"]
    #[doc = " default library will be loaded upon creation of the first Vulkan window."]
    #[doc = ""]
    #[doc = " It is fairly common for Vulkan applications to link with libvulkan instead"]
    #[doc = " of explicitly loading it at run time. This will work with SDL provided the"]
    #[doc = " application links to a dynamic library and both it and SDL use the same"]
    #[doc = " search path."]
    #[doc = ""]
    #[doc = " If you specify a non-NULL `path`, an application should retrieve all of the"]
    #[doc = " Vulkan functions it uses from the dynamic library using"]
    #[doc = " SDL_Vulkan_GetVkGetInstanceProcAddr unless you can guarantee `path` points"]
    #[doc = " to the same vulkan loader library the application linked to."]
    #[doc = ""]
    #[doc = " On Apple devices, if `path` is NULL, SDL will attempt to find the"]
    #[doc = " `vkGetInstanceProcAddr` address within all the Mach-O images of the current"]
    #[doc = " process. This is because it is fairly common for Vulkan applications to"]
    #[doc = " link with libvulkan (and historically MoltenVK was provided as a static"]
    #[doc = " library). If it is not found, on macOS, SDL will attempt to load"]
    #[doc = " `vulkan.framework/vulkan`, `libvulkan.1.dylib`,"]
    #[doc = " `MoltenVK.framework/MoltenVK`, and `libMoltenVK.dylib`, in that order. On"]
    #[doc = " iOS, SDL will attempt to load `libMoltenVK.dylib`. Applications using a"]
    #[doc = " dynamic framework or .dylib must ensure it is included in its application"]
    #[doc = " bundle."]
    #[doc = ""]
    #[doc = " On non-Apple devices, application linking with a static libvulkan is not"]
    #[doc = " supported. Either do not link to the Vulkan loader or link to a dynamic"]
    #[doc = " library version."]
    #[doc = ""]
    #[doc = " \\param path The platform dependent Vulkan loader library name or NULL"]
    #[doc = " \\returns 0 on success or -1 if the library couldn't be loaded; call"]
    #[doc = "          SDL_GetError() for more information."]
    #[doc = ""]
    #[doc = " \\since This function is available in SDL 2.0.8"]
    #[doc = ""]
    #[doc = " \\sa SDL_Vulkan_GetVkInstanceProcAddr"]
    #[doc = " \\sa SDL_Vulkan_UnloadLibrary"]
    pub fn SDL_Vulkan_LoadLibrary(path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Get the address of the `vkGetInstanceProcAddr` function."]
    #[doc = ""]
    #[doc = " This should be called after either calling SDL_Vulkan_LoadLibrary() or"]
    #[doc = " creating an SDL_Window with the `SDL_WINDOW_VULKAN` flag."]
    #[doc = ""]
    #[doc = " \\returns the function pointer for `vkGetInstanceProcAddr` or NULL on error."]
    pub fn SDL_Vulkan_GetVkGetInstanceProcAddr() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Unload the Vulkan library previously loaded by SDL_Vulkan_LoadLibrary()"]
    #[doc = ""]
    #[doc = " \\since This function is available in SDL 2.0.8"]
    #[doc = ""]
    #[doc = " \\sa SDL_Vulkan_LoadLibrary"]
    pub fn SDL_Vulkan_UnloadLibrary();
}
extern "C" {
    #[doc = " Get the names of the Vulkan instance extensions needed to create a surface"]
    #[doc = " with SDL_Vulkan_CreateSurface."]
    #[doc = ""]
    #[doc = " If `pNames` is NULL, then the number of required Vulkan instance extensions"]
    #[doc = " is returned in `pCount`. Otherwise, `pCount` must point to a variable set"]
    #[doc = " to the number of elements in the `pNames` array, and on return the variable"]
    #[doc = " is overwritten with the number of names actually written to `pNames`. If"]
    #[doc = " `pCount` is less than the number of required extensions, at most `pCount`"]
    #[doc = " structures will be written. If `pCount` is smaller than the number of"]
    #[doc = " required extensions, SDL_FALSE will be returned instead of SDL_TRUE, to"]
    #[doc = " indicate that not all the required extensions were returned."]
    #[doc = ""]
    #[doc = " The `window` parameter is currently needed to be valid as of SDL 2.0.8,"]
    #[doc = " however, this parameter will likely be removed in future releases"]
    #[doc = ""]
    #[doc = " \\param window A window for which the required Vulkan instance extensions"]
    #[doc = "               should be retrieved (will be deprecated in a future release)"]
    #[doc = " \\param pCount A pointer to an unsigned int corresponding to the number of"]
    #[doc = "               extensions to be returned"]
    #[doc = " \\param pNames NULL or a pointer to an array to be filled with required"]
    #[doc = "               Vulkan instance extensions"]
    #[doc = " \\returns SDL_TRUE on success, SDL_FALSE on error."]
    #[doc = ""]
    #[doc = " \\since This function is available in SDL 2.0.8"]
    #[doc = ""]
    #[doc = " \\sa SDL_Vulkan_CreateSurface"]
    pub fn SDL_Vulkan_GetInstanceExtensions(
        window: *mut SDL_Window,
        pCount: *mut ::std::os::raw::c_uint,
        pNames: *mut *const ::std::os::raw::c_char,
    ) -> SDL_bool;
}
extern "C" {
    #[doc = " Create a Vulkan rendering surface for a window."]
    #[doc = ""]
    #[doc = " The `window` must have been created with the `SDL_WINDOW_VULKAN` flag and"]
    #[doc = " `instance` must have been created with extensions returned by"]
    #[doc = " SDL_Vulkan_GetInstanceExtensions() enabled."]
    #[doc = ""]
    #[doc = " \\param window The window to which to attach the Vulkan surface"]
    #[doc = " \\param instance The Vulkan instance handle"]
    #[doc = " \\param surface A pointer to a VkSurfaceKHR handle to output the newly"]
    #[doc = "                created surface"]
    #[doc = " \\returns SDL_TRUE on success, SDL_FALSE on error."]
    #[doc = ""]
    #[doc = " \\since This function is available in SDL 2.0.8"]
    #[doc = ""]
    #[doc = " \\sa SDL_Vulkan_GetInstanceExtensions"]
    #[doc = " \\sa SDL_Vulkan_GetDrawableSize"]
    pub fn SDL_Vulkan_CreateSurface(
        window: *mut SDL_Window,
        instance: Instance,
        surface: *mut SurfaceKHR,
    ) -> SDL_bool;
}
extern "C" {
    #[doc = " Get the size of the window's underlying drawable dimensions in pixels."]
    #[doc = ""]
    #[doc = " This may differ from SDL_GetWindowSize() if we're rendering to a high-DPI"]
    #[doc = " drawable, i.e. the window was created with `SDL_WINDOW_ALLOW_HIGHDPI` on a"]
    #[doc = " platform with high-DPI support (Apple calls this \"Retina\"), and not"]
    #[doc = " disabled by the `SDL_HINT_VIDEO_HIGHDPI_DISABLED` hint."]
    #[doc = ""]
    #[doc = " \\param window an SDL_Window for which the size is to be queried"]
    #[doc = " \\param w Pointer to the variable to write the width to or NULL"]
    #[doc = " \\param h Pointer to the variable to write the height to or NULL"]
    #[doc = ""]
    #[doc = " \\since This function is available in SDL 2.0.8"]
    #[doc = ""]
    #[doc = " \\sa SDL_GetWindowSize"]
    #[doc = " \\sa SDL_CreateWindow"]
    #[doc = " \\sa SDL_Vulkan_CreateSurface"]
    pub fn SDL_Vulkan_GetDrawableSize(
        window: *mut SDL_Window,
        w: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
    );
}
