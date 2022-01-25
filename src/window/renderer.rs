use core::ptr;
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::os::raw::{c_char, c_uint};
use std::ptr::{null, null_mut};

use ash::{Device, Entry, Instance};
use ash::extensions::khr::Surface;
use ash::vk::{
    API_VERSION_1_1, ApplicationInfo, DeviceCreateInfo, DeviceQueueCreateInfo, InstanceCreateInfo,
    make_api_version, MAX_EXTENSION_NAME_SIZE, PFN_vkCreateInstance, PhysicalDevice, PhysicalDeviceFeatures,
    PhysicalDeviceType, QueueFlags, StructureType, SurfaceKHR, TRUE,
};

use sdl::{SDL_CreateRenderer, SDL_GetError, SDL_Renderer, SDL_RendererFlags_SDL_RENDERER_ACCELERATED, SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC, SDL_RendererFlags_SDL_RENDERER_SOFTWARE, SDL_RendererFlags_SDL_RENDERER_TARGETTEXTURE, SDL_Window, vulkan};
use sdl::vulkan::{SDL_Vulkan_CreateSurface, SDL_Vulkan_GetVkGetInstanceProcAddr};

#[repr(u32)]
pub enum RendererFlags {
    Software = SDL_RendererFlags_SDL_RENDERER_SOFTWARE,
    Accelerated = SDL_RendererFlags_SDL_RENDERER_ACCELERATED,
    PresentVSync = SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC,
    TargetTexture = SDL_RendererFlags_SDL_RENDERER_TARGETTEXTURE,
}

pub struct Renderer {
    renderer: *mut SDL_Renderer,
    vk_surface: SurfaceKHR,
}

struct QueueFamilyIndex {
    pub graphic: u32,
    pub present: u32,
}

impl Renderer {
    // TODO Remove this usage of raw pointers
    pub fn new(window: *mut SDL_Window) -> Self {

        let vk_entry = unsafe { Entry::load() }.expect("Unable to load Vulkan");
        let vk_instance = Self::create_instance(window, &vk_entry);

        // TODO Create a debug pipeline
        let vk_surface_khr = unsafe {

            let mut surface = SurfaceKHR::default();

            let result =
                sdl::vulkan::SDL_Vulkan_CreateSurface(window, vk_instance.handle(), &mut surface);

            if result == 0 {
                let error = CStr::from_ptr(SDL_GetError()).to_str().unwrap();

                panic!("Failed to create Vulkan surface: {}", error);
            }

            surface
        };

        let surface = Surface::new(&vk_entry, &vk_instance);


        let vk_device = Self::create_physical_device(&vk_instance);

        let vk_queue_families =
            Self::create_queue_family(&surface, &vk_instance, &vk_device, &vk_surface_khr);

        let device = Self::create_device(&vk_instance, &vk_device, &vk_queue_families);

        Self {
            renderer: null_mut(),
            vk_surface: vk_surface_khr,
        }
    }

    fn create_physical_device(instance: &Instance) -> PhysicalDevice {
        unsafe {
            let result = instance
                .enumerate_physical_devices()
                .expect("Cannot enumerate physical devices");

            result
                .into_iter()
                .find(|d| Self::check_suitability(&instance, d))
                .expect("No graphical card found!")
        }
    }

    fn create_device(instance: &Instance, physical_device: &PhysicalDevice, queue_families: &QueueFamilyIndex) -> Device {
        let x = [queue_families.graphic, queue_families.present];
        let families = x
            .iter()
            .collect::<HashSet<_>>();

        let priority = 1.0f32;

        let map = families
            .iter()
            .map(|f| DeviceQueueCreateInfo {
                s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,
                queue_family_index: **f,
                queue_count: 1,
                p_queue_priorities: &priority,
                ..Default::default()
            })
            .collect::<Vec<_>>();

        // TODO Use CString instead
        let device_extensions = unsafe { vec![CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0") ]};

        let device_extensions = device_extensions
            .into_iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        let physical_device_features = [PhysicalDeviceFeatures {
            sampler_anisotropy: TRUE,
            ..Default::default()
        }];

        let create_info = DeviceCreateInfo {
            s_type: StructureType::DEVICE_CREATE_INFO,
            p_queue_create_infos: map.as_ptr(),
            queue_create_info_count: map.len() as u32,
            p_enabled_features: physical_device_features.as_ptr(),
            enabled_extension_count: device_extensions.len() as u32,
            pp_enabled_extension_names: device_extensions.as_ptr(),
            // TODO Add validation layers
            ..Default::default()
        };

        unsafe { instance.create_device(*physical_device, &create_info, None).expect("Unable to create device") }
    }

    fn create_queue_family(
        surface: &Surface,
        instance: &Instance,
        device: &PhysicalDevice,
        surface_khr: &SurfaceKHR,
    ) -> QueueFamilyIndex {
        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(*device) };

        let graphics_queue = queue_family_properties
            .iter()
            .enumerate()
            .find(|(i, q)| q.queue_flags.contains(QueueFlags::GRAPHICS))
            .map(|(i, _)| i as u32)
            .expect("No suitable graphics queue family found");

        let present_queue = queue_family_properties
            .into_iter()
            .enumerate()
            .find(|(i, q)| unsafe {
                surface
                    .get_physical_device_surface_support(*device, *i as u32, *surface_khr)
                    .is_ok()
            })
            .map(|(i, _)| i as u32)
            .expect("No suitable present queue family found");

        QueueFamilyIndex {
            graphic: graphics_queue,
            present: present_queue,
        }
    }

    fn check_suitability(_instance: &Instance, _device: &PhysicalDevice) -> bool {
        // let properties = unsafe { instance.get_physical_device_properties(*device) };
        // let features = unsafe { instance.get_physical_device_features(*device) };
        //
        // let name = unsafe {
        //     CStr::from_ptr(properties.device_name.as_ptr())
        //         .to_str()
        //         .unwrap()
        // };

        // TODO Implement a verification
        true
    }

    fn get_required_extensions(window: *mut SDL_Window) -> Vec<*const c_char> {
        let mut enabled_extension_count = c_uint::from(0u16);

        unsafe {
            sdl::vulkan::SDL_Vulkan_GetInstanceExtensions(
                window,
                &mut enabled_extension_count,
                null_mut(),
            );
        }

        let mut extension_names = Vec::<*const c_char>::new();
        extension_names.resize(enabled_extension_count as usize, null());

        unsafe {
            sdl::vulkan::SDL_Vulkan_GetInstanceExtensions(
                window,
                &mut enabled_extension_count,
                extension_names.as_mut_ptr(),
            );
        }

        extension_names
    }

    fn create_instance(window: *mut SDL_Window, entry: &Entry) -> ash::Instance {
        // TODO Pass by function parameter
        let app_name = "";

        let app_info = ApplicationInfo {
            s_type: StructureType::APPLICATION_INFO,
            p_application_name: app_name.as_ptr() as *const c_char,
            application_version: make_api_version(1, 1, 0, 0),
            p_engine_name: "No Engine".as_ptr() as *const c_char,
            engine_version: make_api_version(1, 0, 0, 0),
            api_version: API_VERSION_1_1,
            ..Default::default()
        };

        let required_extensions = Self::get_required_extensions(window);

        let app_create_info = InstanceCreateInfo {
            s_type: StructureType::INSTANCE_CREATE_INFO,
            p_application_info: &app_info,
            pp_enabled_extension_names: required_extensions.as_ptr(),
            enabled_extension_count: required_extensions.len() as u32,
            ..Default::default()
        };

        unsafe { entry.create_instance(&app_create_info, None) }
            .expect("Cannot create Vulkan instance")
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // TODO Drop Vulkan surface
    }

}