use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::{null, null_mut};

use ash::{Device, Entry, Instance};
use ash::extensions::khr::{Surface, Swapchain};
use ash::vk::{
    ApplicationInfo, CompositeAlphaFlagsKHR, DeviceCreateInfo, DeviceQueueCreateInfo,
    Extent2D, Format, Image, ImageAspectFlags, ImageSubresourceRange, ImageUsageFlags,
    ImageView, ImageViewCreateInfo, ImageViewType, InstanceCreateInfo, KhrSwapchainFn,
    make_api_version, PhysicalDevice, PhysicalDeviceFeatures, PresentModeKHR, QueueFlags,
    SharingMode, StructureType, SurfaceFormatKHR, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR,
};

use sdl::{SDL_GetError, SDL_Window};
use sdl::vulkan::SDL_Vulkan_GetDrawableSize;

#[derive(Copy, Clone)]
pub struct QueueFamilyIndex {
    pub graphic: u32,
    pub present: u32,
}

pub struct Pipeline {
    pub entry: Entry,
    pub instance: Instance,
    pub surface: Surface,
    pub surface_khr: SurfaceKHR,
    pub physical_device: PhysicalDevice,
    pub queue_families: QueueFamilyIndex,
    pub device: Device,
    pub swapchain_khr: SwapchainKHR,
    pub surface_format_khr: SurfaceFormatKHR,
    pub images: Vec<Image>,
    pub image_views: Vec<ImageView>,
}

impl Pipeline {
    pub fn from_sdl_window(window: &mut SDL_Window) -> Self {
        let entry = unsafe { Entry::load() }.expect("Unable to load Vulkan");

        let instance = Self::create_instance(window, &entry);

        // TODO Create a debug pipeline
        let surface_khr = Self::create_surface_khr(window, &instance);

        let surface = Surface::new(&entry, &instance);

        let physical_device = Self::create_physical_device(&instance);

        let queue_families =
            Self::create_queue_family(&surface, &instance, physical_device, surface_khr);

        let device = Self::create_device(&instance, physical_device, queue_families);

        let (swapchain_khr, surface_format_khr, images) = Self::create_swapchain(
            &instance,
            &device,
            physical_device,
            &surface,
            surface_khr,
            window,
            queue_families,
        );

        let image_views = Self::create_image_views(&device, surface_format_khr, images.clone());

        Self {
            entry,
            instance,
            surface,
            surface_khr,
            physical_device,
            queue_families,
            device,
            swapchain_khr,
            surface_format_khr,
            images,
            image_views,
        }
    }

    fn create_surface_khr(window: &mut SDL_Window, instance: &Instance) -> SurfaceKHR {
        let surface_khr = unsafe {
            let mut surface = SurfaceKHR::default();

            let result =
                sdl::vulkan::SDL_Vulkan_CreateSurface(window, instance.handle(), &mut surface);

            if result == 0 {
                let error = CStr::from_ptr(SDL_GetError()).to_str().unwrap();

                panic!("Failed to create Vulkan surface: {}", error);
            }

            surface
        };
        surface_khr
    }

    fn create_physical_device(instance: &Instance) -> PhysicalDevice {
        unsafe {
            let result = instance
                .enumerate_physical_devices()
                .expect("Cannot enumerate physical devices");

            result
                .into_iter()
                .find(|&d| Self::check_suitability(&instance, d))
                .expect("No graphical card found!")
        }
    }

    fn create_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        queue_families: QueueFamilyIndex,
    ) -> Device {
        let families = HashSet::from([queue_families.graphic, queue_families.present]);

        let priority = 1.0f32;

        let map = families
            .into_iter()
            .map(|f| DeviceQueueCreateInfo {
                s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,
                queue_family_index: f,
                queue_count: 1,
                p_queue_priorities: &priority,
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let device_extensions = vec![KhrSwapchainFn::name()];

        let device_extensions = device_extensions
            .into_iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        let physical_device_features = [PhysicalDeviceFeatures {
            sampler_anisotropy: ash::vk::TRUE,
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

        unsafe {
            instance
                .create_device(physical_device, &create_info, None)
                .expect("Unable to create device")
        }
    }

    fn create_queue_family(
        surface: &Surface,
        instance: &Instance,
        device: PhysicalDevice,
        surface_khr: SurfaceKHR,
    ) -> QueueFamilyIndex {
        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(device) };

        let graphics_queue = queue_family_properties
            .iter()
            .enumerate()
            .find(|(_, q)| q.queue_flags.contains(QueueFlags::GRAPHICS))
            .map(|(i, _)| i as u32)
            .expect("No suitable graphics queue family found");

        let present_queue = (0..queue_family_properties.len() as u32)
            .find(|i| unsafe {
                surface
                    .get_physical_device_surface_support(device, *i as u32, surface_khr)
                    .is_ok()
            })
            .expect("No suitable present queue family found");

        QueueFamilyIndex {
            graphic: graphics_queue,
            present: present_queue,
        }
    }

    fn check_suitability(_instance: &Instance, _device: PhysicalDevice) -> bool {
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

    fn get_required_extensions(window: &mut SDL_Window) -> Vec<String> {
        let mut enabled_extension_count = 0u32;

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

        unsafe {
            extension_names
                .into_iter()
                .map(|e| CStr::from_ptr(e).to_str().unwrap().to_owned())
                .collect()
        }
    }

    fn create_instance(window: &mut SDL_Window, entry: &Entry) -> ash::Instance {
        // TODO Pass by function parameter
        let app_name = "";

        let app_info = ApplicationInfo {
            s_type: StructureType::APPLICATION_INFO,
            p_application_name: app_name.as_ptr() as *const c_char,
            application_version: make_api_version(1, 1, 0, 0),
            p_engine_name: "No Engine".as_ptr() as *const c_char,
            engine_version: make_api_version(1, 0, 0, 0),
            api_version: ash::vk::API_VERSION_1_1,
            ..Default::default()
        };

        // Holds the ownership of string values and must be NOT deleted
        let required_extensions = Self::get_required_extensions(window)
            .into_iter()
            .map(|e| CString::new(e).unwrap())
            .collect::<Vec<_>>();

        let required_extensions_cchar = required_extensions
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        let app_create_info = InstanceCreateInfo {
            s_type: StructureType::INSTANCE_CREATE_INFO,
            p_application_info: &app_info,
            pp_enabled_extension_names: required_extensions_cchar.as_ptr(),
            enabled_extension_count: required_extensions_cchar.len() as u32,
            ..Default::default()
        };

        unsafe { entry.create_instance(&app_create_info, None) }
            .expect("Cannot create Vulkan instance")
    }

    #[must_use]
    fn create_swapchain(
        instance: &Instance,
        device: &Device,
        physical_device: PhysicalDevice,
        surface: &Surface,
        surface_khr: SurfaceKHR,
        window: &mut SDL_Window,
        queue_family_index: QueueFamilyIndex,
    ) -> (SwapchainKHR, SurfaceFormatKHR, Vec<Image>) {
        let surface_formats = unsafe {
            surface
                .get_physical_device_surface_formats(physical_device, surface_khr)
                .expect("Cannot get physical device surface formats")
        };

        let surface_capabilities = unsafe {
            surface
                .get_physical_device_surface_capabilities(physical_device, surface_khr)
                .expect("Cannot get surface capabilities")
        };

        // surface_formats
        //     .iter()
        //     .for_each(|f| {
        //        println!("{:?}", f);
        //     });

        let surface_format = surface_formats
            .into_iter()
            .find(|f| f.format != Format::B8G8R8A8_UNORM)
            .expect("Physical device does not support required surface format");

        let mut width = 0i32;
        let mut height = 0i32;

        unsafe { SDL_Vulkan_GetDrawableSize(window, &mut width, &mut height) };

        let width = (width as u32).clamp(
            surface_capabilities.min_image_extent.width,
            surface_capabilities.max_image_extent.width,
        );

        let height = (height as u32).clamp(
            surface_capabilities.min_image_extent.height,
            surface_capabilities.max_image_extent.height,
        );

        let swapchain_size = Extent2D {
            width,
            height,
            ..Default::default()
        };

        let queues = if queue_family_index.graphic == queue_family_index.present {
            vec![queue_family_index.graphic, queue_family_index.present]
        } else {
            vec![queue_family_index.graphic]
        };

        let create_info = SwapchainCreateInfoKHR {
            s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            surface: surface_khr,
            min_image_count: surface_capabilities.min_image_count,
            image_format: surface_format.format,
            image_color_space: surface_format.color_space,
            image_extent: swapchain_size,
            image_array_layers: 1,
            image_usage: ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode: match queues.len() {
                1 => SharingMode::EXCLUSIVE,
                2 => SharingMode::CONCURRENT,
                _ => unreachable!(),
            },
            queue_family_index_count: queues.len() as u32,
            p_queue_family_indices: queues.as_ptr(),
            pre_transform: surface_capabilities.current_transform,
            composite_alpha: CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: PresentModeKHR::FIFO,
            clipped: ash::vk::TRUE,
            ..Default::default()
        };

        let swapchain = Swapchain::new(instance, device);

        let swapchain_khr = unsafe {
            swapchain
                .create_swapchain(&create_info, None)
                .expect("Cannot create swapchain")
        };

        let swapchain_images = unsafe {
            swapchain
                .get_swapchain_images(swapchain_khr.clone())
                .expect("Cannot get swapchain images")
        };

        (swapchain_khr, surface_format, swapchain_images)
    }

    fn create_image_views(
        device: &Device,
        surface_format_khr: SurfaceFormatKHR,
        images: Vec<Image>,
    ) -> Vec<ImageView> {
        images
            .iter()
            .map(|image| unsafe {
                let create_info = ImageViewCreateInfo {
                    s_type: StructureType::IMAGE_VIEW_CREATE_INFO,
                    image: *image,
                    view_type: ImageViewType::TYPE_2D,
                    format: surface_format_khr.format,
                    subresource_range: ImageSubresourceRange {
                        aspect_mask: ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                        ..Default::default()
                    },
                    ..Default::default()
                };
                device
                    .create_image_view(&create_info, None)
                    .expect("Cannot create image view")
            })
            .collect::<Vec<_>>()
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            self.image_views
                .iter()
                .for_each(|iv| self.device.destroy_image_view(*iv, None));

            self.images
                .iter()
                .for_each(|i| self.device.destroy_image(*i, None));

            self.device.destroy_device(None);
            self.surface.destroy_surface(self.surface_khr, None);
            self.instance.destroy_instance(None);
        }
    }
}
