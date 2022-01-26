use core::ptr;
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::os::raw::{c_char, c_uint};
use std::ptr::{null, null_mut};

use ash::{Device, Entry, Instance};
use ash::extensions::khr::{Surface, Swapchain};
use ash::vk::{
    API_VERSION_1_1, ApplicationInfo, CompositeAlphaFlagsKHR, DeviceCreateInfo,
    DeviceQueueCreateInfo, Extent2D, Format, Image, ImageAspectFlags, ImageSubresourceRange,
    ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType, InstanceCreateInfo,
    make_api_version, MAX_EXTENSION_NAME_SIZE, PFN_vkCreateInstance, PhysicalDevice,
    PhysicalDeviceFeatures, PhysicalDeviceType, PresentModeKHR, QueueFlags, SharingMode, StructureType,
    SurfaceFormatKHR, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR, TRUE,
};

use sdl::{
    SDL_CreateRenderer, SDL_GetError, SDL_Renderer, SDL_RendererFlags_SDL_RENDERER_ACCELERATED,
    SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC, SDL_RendererFlags_SDL_RENDERER_SOFTWARE,
    SDL_RendererFlags_SDL_RENDERER_TARGETTEXTURE, SDL_Window,
    vulkan,
};
use sdl::vulkan::{
    SDL_Vulkan_CreateSurface, SDL_Vulkan_GetDrawableSize, SDL_Vulkan_GetVkGetInstanceProcAddr,
};

use crate::Window;
use crate::window::pipeline::Pipeline;

#[repr(u32)]
pub enum RendererFlags {
    Software = SDL_RendererFlags_SDL_RENDERER_SOFTWARE,
    Accelerated = SDL_RendererFlags_SDL_RENDERER_ACCELERATED,
    PresentVSync = SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC,
    TargetTexture = SDL_RendererFlags_SDL_RENDERER_TARGETTEXTURE,
}

pub struct Renderer {
    pipeline: Pipeline,
}

impl Renderer {
    pub fn new(window: &mut SDL_Window) -> Self {

        let pipeline = Pipeline::from_sdl_window(window);

        Self {
            pipeline,
        }
    }
}
