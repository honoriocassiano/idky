use std::ffi::{c_void, CStr, CString};
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;
use std::path::Path;

use ash::{Device, Entry, Instance};
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::{Surface, Swapchain};
use ash::vk::{
    ApplicationInfo, Bool32, Buffer, BufferCreateInfo, BufferUsageFlags, CompositeAlphaFlagsKHR,
    CullModeFlags, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
    DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessengerEXT,
    DeviceCreateInfo, DeviceMemory, DeviceQueueCreateInfo, DeviceSize, Extent2D, Extent3D, Filter,
    Format, FrontFace, Image, ImageAspectFlags, ImageCreateInfo, ImageLayout,
    ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, ImageView, ImageViewCreateInfo,
    ImageViewType, InstanceCreateInfo, KhrPortabilitySubsetFn, KhrSwapchainFn, LogicOp,
    MemoryAllocateInfo, MemoryMapFlags, MemoryPropertyFlags, Offset2D, PhysicalDevice,
    PhysicalDeviceFeatures, PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo,
    PipelineInputAssemblyStateCreateInfo, PipelineLayoutCreateInfo,
    PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
    PipelineShaderStageCreateInfo, PipelineVertexInputStateCreateInfo,
    PipelineViewportStateCreateInfo, PolygonMode, PresentModeKHR, PrimitiveTopology, QueueFlags,
    Rect2D, SampleCountFlags, Sampler, SamplerAddressMode, SamplerCreateInfo, ShaderModule,
    ShaderModuleCreateInfo, ShaderStageFlags, SharingMode, SurfaceFormatKHR, SurfaceKHR,
    SwapchainCreateInfoKHR, SwapchainKHR, Viewport,
};

use sdl::SDL_Window;

#[derive(Copy, Clone)]
pub struct QueueFamilyIndex {
    pub graphic: u32,
    pub present: u32,
}

impl Into<Vec<u32>> for QueueFamilyIndex {
    fn into(self) -> Vec<u32> {
        self.to_vec()
    }
}

impl QueueFamilyIndex {
    fn to_vec(&self) -> Vec<u32> {
        match self.graphic == self.present {
            false => vec![self.graphic, self.present],
            true => vec![self.graphic],
        }
    }
}

pub struct Pipeline {
    pub entry: Entry,
    pub instance: Instance,
    pub surface: Surface,
    pub surface_khr: SurfaceKHR,
    pub physical_device: PhysicalDevice,
    pub queue_families: QueueFamilyIndex,
    pub device: Device,
    pub swapchain: Swapchain,
    pub swapchain_extent: Extent2D,
    pub swapchain_khr: SwapchainKHR,
    pub surface_format_khr: SurfaceFormatKHR,
    pub swapchain_images: Vec<Image>,
    pub image_views: Vec<ImageView>,
    pub samplers: Vec<Sampler>,
    #[cfg(debug_assertions)]
    pub debug_utils: DebugUtils,
    #[cfg(debug_assertions)]
    pub debug_utils_messenger: DebugUtilsMessengerEXT,
}

impl Pipeline {
    pub fn from_sdl_window(window: &mut SDL_Window) -> Self {
        let entry = unsafe { Entry::load() }.expect("Unable to load Vulkan");

        let instance = Self::create_instance(window, &entry);

        #[cfg(debug_assertions)]
        let debug_utils = DebugUtils::new(&entry, &instance);

        #[cfg(debug_assertions)]
        let debug_utils_messenger = Self::setup_debug_messenger(&debug_utils);

        // TODO Create a debug pipeline
        let surface_khr = Self::create_surface_khr(window, &instance);

        let surface = Surface::new(&entry, &instance);

        let (physical_device, additional_extensions) = Self::create_physical_device(&instance);

        let queue_families =
            Self::get_queue_families(&surface, &instance, physical_device, surface_khr);

        let device = Self::create_device(
            &instance,
            physical_device,
            queue_families,
            additional_extensions.as_slice(),
        );

        let (swapchain, swapchain_extent, swapchain_khr, surface_format_khr, swapchain_images) =
            Self::create_swapchain(
                &instance,
                &device,
                physical_device,
                &surface,
                surface_khr,
                window,
                queue_families,
            );

        let image_views =
            Self::create_image_views(&device, surface_format_khr, swapchain_images.clone());

        let samplers = vec![Self::create_sampler(&device)];

        Self {
            entry,
            instance,
            surface,
            surface_khr,
            physical_device,
            queue_families,
            device,
            swapchain,
            swapchain_extent,
            swapchain_khr,
            surface_format_khr,
            swapchain_images,
            image_views,
            samplers,
            #[cfg(debug_assertions)]
            debug_utils,
            #[cfg(debug_assertions)]
            debug_utils_messenger,
        }
    }

    #[cfg(debug_assertions)]
    fn get_validation_layers() -> Vec<&'static CStr> {
        let layer =
            unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") };

        vec![layer]
    }

    fn create_surface_khr(window: &mut SDL_Window, instance: &Instance) -> SurfaceKHR {
        let surface_khr = unsafe {
            let mut surface = SurfaceKHR::default();

            let result =
                sdl::vulkan::SDL_Vulkan_CreateSurface(window, instance.handle(), &mut surface);

            if result == 0 {
                let error = CStr::from_ptr(sdl::SDL_GetError()).to_str().unwrap();

                panic!("Failed to create Vulkan surface: {}", error);
            }

            surface
        };
        surface_khr
    }

    fn create_physical_device(instance: &Instance) -> (PhysicalDevice, Vec<&'static CStr>) {
        unsafe {
            let result = instance
                .enumerate_physical_devices()
                .expect("Cannot enumerate physical devices");

            let device = result
                .into_iter()
                .find(|&d| Self::check_suitability(&instance, d))
                .expect("No graphical card found!");

            let extension_properties = instance
                .enumerate_device_extension_properties(device)
                .expect("Unable to enumerate physical device extension properties");

            let portability_subset = extension_properties.into_iter().find(|ep| {
                CStr::from_ptr(ep.extension_name.as_ptr()) == KhrPortabilitySubsetFn::name()
            });

            match portability_subset {
                None => (device, vec![]),
                Some(_) => (device, vec![KhrPortabilitySubsetFn::name()]),
            }
        }
    }

    fn create_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        queue_families: QueueFamilyIndex,
        additional_extensions: &[&'static CStr],
    ) -> Device {
        let families: Vec<u32> = queue_families.into();

        let priority = 1.0f32;

        let map = families
            .into_iter()
            .map(|f| {
                DeviceQueueCreateInfo::builder()
                    .queue_family_index(f)
                    .queue_priorities(&[priority])
                    .build()
            })
            .collect::<Vec<_>>();

        let device_extensions = [&[KhrSwapchainFn::name()], additional_extensions]
            .concat()
            .iter()
            .map(|de| de.as_ptr())
            .collect::<Vec<_>>();

        // TODO Add validation layers if supported

        let physical_device_features = PhysicalDeviceFeatures::builder()
            .sampler_anisotropy(true)
            .build();

        let create_info = DeviceCreateInfo::builder()
            .queue_create_infos(map.as_slice())
            .enabled_features(&physical_device_features)
            .enabled_extension_names(device_extensions.as_slice())
            .build();

        unsafe {
            instance
                .create_device(physical_device, &create_info, None)
                .expect("Unable to create device")
        }
    }

    fn get_queue_families(
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
            .find(|&i| unsafe {
                surface
                    .get_physical_device_surface_support(device, i as u32, surface_khr)
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
                std::ptr::null_mut(),
            );
        }

        let mut extension_names = Vec::<*const c_char>::new();
        extension_names.resize(enabled_extension_count as usize, std::ptr::null());

        unsafe {
            sdl::vulkan::SDL_Vulkan_GetInstanceExtensions(
                window,
                &mut enabled_extension_count,
                extension_names.as_mut_ptr(),
            );
        }

        #[cfg(debug_assertions)]
        {
            extension_names.push(DebugUtils::name().as_ptr());
        }

        unsafe {
            extension_names
                .into_iter()
                .map(|e| CStr::from_ptr(e).to_str().unwrap().to_owned())
                .collect()
        }
    }

    #[cfg(debug_assertions)]
    extern "system" fn debug_callback(
        _message_severity: DebugUtilsMessageSeverityFlagsEXT,
        _message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
        _p_user_data: *mut c_void,
    ) -> Bool32 {
        let message = unsafe { CStr::from_ptr((*p_callback_data).p_message).to_str() };

        // TODO Add a proper log
        match message {
            Ok(m) => println!("{}", m),
            Err(m) => println!("{}", m),
        };

        ash::vk::FALSE
    }

    #[cfg(debug_assertions)]
    fn setup_debug_messenger(debug_utils: &DebugUtils) -> DebugUtilsMessengerEXT {
        let create_info = Self::create_debug_message();

        unsafe {
            debug_utils
                .create_debug_utils_messenger(&create_info, None)
                .expect("Unable to create debug messenger")
        }
    }

    #[cfg(debug_assertions)]
    fn create_debug_message() -> DebugUtilsMessengerCreateInfoEXT {
        DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(Self::debug_callback))
            .build()
    }

    fn create_instance(window: &mut SDL_Window, entry: &Entry) -> ash::Instance {
        // TODO Pass by function parameter
        let app_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };
        let engine_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"No Engine\0") };

        let app_info = ApplicationInfo::builder()
            .application_name(app_name)
            .application_version(ash::vk::make_api_version(1, 1, 0, 0))
            .engine_name(engine_name)
            .engine_version(ash::vk::make_api_version(1, 0, 0, 0))
            .api_version(ash::vk::API_VERSION_1_1)
            .build();

        // Holds the ownership of string values and must be NOT deleted
        let required_extensions = Self::get_required_extensions(window)
            .into_iter()
            .map(|e| CString::new(e).unwrap())
            .collect::<Vec<_>>();

        let required_extensions_cchar = required_extensions
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        #[cfg(debug_assertions)]
        {
            if !Self::check_validation_layers(entry) {
                panic!("Validation layers not available");
            }

            let layer_names = Self::get_validation_layers()
                .iter()
                .map(|vl| vl.as_ptr())
                .collect::<Vec<_>>();

            let mut create_info_ext = Self::create_debug_message();

            let app_create_info = InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_extension_names(required_extensions_cchar.as_slice())
                .enabled_layer_names(layer_names.as_slice())
                .push_next(&mut create_info_ext)
                .build();

            unsafe { entry.create_instance(&app_create_info, None) }
                .expect("Cannot create Vulkan instance")
        }

        #[cfg(not(debug_assertions))]
        {
            let app_create_info = InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_extension_names(required_extensions_cchar.as_slice())
                .enabled_layer_count(0)
                .build();

            unsafe { entry.create_instance(&app_create_info, None) }
                .expect("Cannot create Vulkan instance")
        }
    }

    #[cfg(debug_assertions)]
    #[must_use]
    fn check_validation_layers(entry: &Entry) -> bool {
        let layers = entry
            .enumerate_instance_layer_properties()
            .expect("Unable to enumerate layer properties");

        let layer = layers
            .into_iter()
            .filter(|l| unsafe {
                let ptr = CStr::from_ptr(l.layer_name.as_ptr());

                Self::get_validation_layers()
                    .iter()
                    .find(|&&vl| ptr == vl)
                    .is_some()
            })
            .collect::<Vec<_>>();

        !layer.is_empty()
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
    ) -> (
        Swapchain,
        Extent2D,
        SwapchainKHR,
        SurfaceFormatKHR,
        Vec<Image>,
    ) {
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

        unsafe { sdl::vulkan::SDL_Vulkan_GetDrawableSize(window, &mut width, &mut height) };

        let width = (width as u32).clamp(
            surface_capabilities.min_image_extent.width,
            surface_capabilities.max_image_extent.width,
        );

        let height = (height as u32).clamp(
            surface_capabilities.min_image_extent.height,
            surface_capabilities.max_image_extent.height,
        );

        let swapchain_extent = Extent2D {
            width,
            height,
            ..Default::default()
        };

        let queues: Vec<u32> = queue_family_index.into();
        let sharing_mode = match queues.len() {
            1 => SharingMode::EXCLUSIVE,
            2 => SharingMode::CONCURRENT,
            _ => unreachable!(),
        };

        let create_info = SwapchainCreateInfoKHR::builder()
            .surface(surface_khr)
            .min_image_count(surface_capabilities.min_image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(swapchain_extent)
            .image_array_layers(1)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(sharing_mode)
            .queue_family_indices(queues.as_slice())
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(PresentModeKHR::FIFO)
            .clipped(true)
            .build();

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

        (
            swapchain,
            swapchain_extent,
            swapchain_khr,
            surface_format,
            swapchain_images,
        )
    }

    fn create_image_views(
        device: &Device,
        surface_format_khr: SurfaceFormatKHR,
        images: Vec<Image>,
    ) -> Vec<ImageView> {
        images
            .iter()
            .map(|image| unsafe {
                let subresource_range = ImageSubresourceRange::builder()
                    .aspect_mask(ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build();

                let create_info = ImageViewCreateInfo::builder()
                    .image(*image)
                    .view_type(ImageViewType::TYPE_2D)
                    .format(surface_format_khr.format)
                    .subresource_range(subresource_range)
                    .build();

                device
                    .create_image_view(&create_info, None)
                    .expect("Cannot create image view")
            })
            .collect::<Vec<_>>()
    }

    #[allow(unused)]
    pub fn create_image(
        &mut self,
        width: u32,
        height: u32,
        format: Format,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        properties: MemoryPropertyFlags,
        device_memory: DeviceMemory,
    ) {
        let extent = Extent3D {
            width,
            height,
            depth: 1,
            ..Default::default()
        };

        let create_info = ImageCreateInfo::builder()
            .image_type(ImageType::TYPE_2D)
            .extent(extent)
            .mip_levels(1)
            .array_layers(1)
            .format(format)
            .tiling(tiling)
            .initial_layout(ImageLayout::UNDEFINED)
            .usage(usage)
            .samples(SampleCountFlags::TYPE_1)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .build();

        let image = unsafe {
            self.device
                .create_image(&create_info, None)
                .expect("Cannot create image")
        };

        let requirements = unsafe { self.device.get_image_memory_requirements(image) };

        let alloc_info = MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(Self::find_memory_type(
                requirements.memory_type_bits,
                properties,
            ))
            .build();

        unsafe {
            self.device
                .allocate_memory(&alloc_info, None)
                .expect("Cannot allocate image memory");
            self.device
                .bind_image_memory(image, device_memory, 0)
                .expect("Unable to bind image memory");
        }
    }

    #[allow(unused)]
    pub fn create_shader_module<T: AsRef<Path>>(&self, path: T) -> ShaderModule {
        let bytes = Self::read_file(path.as_ref())
            .expect(format!("Cannot read file {}", path.as_ref().display()).as_str());

        let create_info = ShaderModuleCreateInfo::builder()
            .code(unsafe { std::mem::transmute(bytes.as_slice()) })
            .build();

        unsafe {
            self.device
                .create_shader_module(&create_info, None)
                .expect("Cannot create shader module")
        }
    }

    #[allow(unused)]
    fn read_file<T: AsRef<Path>>(path: T) -> std::io::Result<Vec<u8>> {
        let mut bytes = Vec::<u8>::new();

        File::open(path.as_ref())?
            .read_to_end(&mut bytes)?;

        Ok(bytes)
    }

    #[allow(unused)]
    fn create_graphics_pipeline(&self) {
        let vertex_shader = self.create_shader_module("shaders/vert.spv");
        let fragment_shader = self.create_shader_module("shaders/frag.spv");

        let function_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") };

        let _vertex_shader_info = PipelineShaderStageCreateInfo::builder()
            .stage(ShaderStageFlags::VERTEX)
            .module(vertex_shader)
            .name(function_name)
            .build();

        let _fragment_shader_info = PipelineShaderStageCreateInfo::builder()
            .stage(ShaderStageFlags::FRAGMENT)
            .module(fragment_shader)
            .name(function_name)
            .build();

        let _shader_stages = [vertex_shader, fragment_shader];

        let _vertex_input_state_create_info = PipelineVertexInputStateCreateInfo::default();

        // TODO Finish pipeline creation

        let _input_assembly_state_create_info = PipelineInputAssemblyStateCreateInfo::builder()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();

        let viewport = Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(self.swapchain_extent.width as f32)
            .height(self.swapchain_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build();

        let scissor = Rect2D::builder()
            .offset(Offset2D { x: 0, y: 0 })
            .extent(self.swapchain_extent)
            .build();

        let _viewport_create_info = PipelineViewportStateCreateInfo::builder()
            .viewports(&[viewport])
            .scissors(&[scissor])
            .build();

        // TODO Check these values
        let _rasterizer = PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(CullModeFlags::BACK)
            .front_face(FrontFace::CLOCKWISE)
            .depth_bias_enable(false)
            .build();

        // TODO Check these values
        let _multisampling = PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(SampleCountFlags::TYPE_1)
            .build();

        // TODO Check this value
        let color_blend_attachments = PipelineColorBlendAttachmentState::builder()
            .blend_enable(false)
            .build();

        let _color_blending = PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(LogicOp::COPY)
            .attachments(&[color_blend_attachments])
            .blend_constants([0.0, 0.0, 0.0, 0.0])
            .build();

        // TODO Set pipeline layout
        // TODO Set constant ranges
        let pipeline_create_info = PipelineLayoutCreateInfo::builder()
            .set_layouts(&[])
            .push_constant_ranges(&[])
            .build();

        unsafe {
            self.device
                .create_pipeline_layout(&pipeline_create_info, None)
                .expect("Unable to create pipeline layout");

            self.device.destroy_shader_module(vertex_shader, None);
            self.device.destroy_shader_module(fragment_shader, None);
        }
    }

    #[allow(unused)]
    pub fn create_buffer(
        &mut self,
        size: DeviceSize,
        usage: BufferUsageFlags,
        properties: MemoryPropertyFlags,
    ) -> (Buffer, DeviceMemory) {
        let create_info = BufferCreateInfo::builder()
            .size(size)
            .usage(usage)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .build();

        let buffer = unsafe {
            self.device
                .create_buffer(&create_info, None)
                .expect("Cannot create buffer")
        };

        let requirements = unsafe { self.device.get_buffer_memory_requirements(buffer) };

        let alloc_info = MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(Self::find_memory_type(
                requirements.memory_type_bits,
                properties,
            ))
            .build();

        let memory = unsafe {
            self.device
                .allocate_memory(&alloc_info, None)
                .expect("Cannot allocate memory")
        };

        unsafe {
            self.device
                .bind_buffer_memory(buffer, memory, 0)
                .expect("Unable to bind buffer");
        }

        (buffer, memory)
    }

    #[allow(unused)]
    pub fn create_image_texture(&mut self) {
        // TODO Set size according to image
        let width = 0u32;
        let height = 0u32;
        let size = DeviceSize::default();

        let buffer_usage_flags = BufferUsageFlags::TRANSFER_SRC;
        let memory_usage_flags =
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT;

        let (_buffer, device_memory) =
            self.create_buffer(size, buffer_usage_flags, memory_usage_flags);

        let _data = unsafe {
            self.device
                .map_memory(device_memory, 0, size, MemoryMapFlags::empty())
                .expect("Cannot map memory")
        };
        // TODO Fill data
        unsafe {
            self.device.unmap_memory(device_memory);
        }

        self.create_image(
            width,
            height,
            Format::R8G8B8A8_SRGB, // TODO Check this format
            ImageTiling::OPTIMAL,
            ImageUsageFlags::TRANSFER_DST | ImageUsageFlags::SAMPLED,
            MemoryPropertyFlags::DEVICE_LOCAL,
            device_memory,
        );
    }

    fn create_sampler(device: &Device) -> Sampler {
        let create_info = SamplerCreateInfo::builder()
            .mag_filter(Filter::NEAREST)
            .min_filter(Filter::NEAREST)
            .address_mode_u(SamplerAddressMode::REPEAT)
            .address_mode_v(SamplerAddressMode::REPEAT)
            .address_mode_w(SamplerAddressMode::REPEAT)
            .anisotropy_enable(false)
            .max_anisotropy(1.0)
            .build();

        unsafe {
            device
                .create_sampler(&create_info, None)
                .expect("Unable to create sampler")
        }
    }

    #[allow(unused)]
    fn find_memory_type(_type_bits: u32, _properties: MemoryPropertyFlags) -> u32 {
        // TODO
        todo!()
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            self.samplers
                .iter()
                .for_each(|s| self.device.destroy_sampler(*s, None));

            self.image_views
                .iter()
                .for_each(|iv| self.device.destroy_image_view(*iv, None));

            self.swapchain.destroy_swapchain(self.swapchain_khr, None);

            self.debug_utils
                .destroy_debug_utils_messenger(self.debug_utils_messenger, None);

            self.device.destroy_device(None);
            self.surface.destroy_surface(self.surface_khr, None);
            self.instance.destroy_instance(None);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::window::pipeline::QueueFamilyIndex;

    #[test]
    fn should_properly_convert_queue_family_index_to_vec() {
        let index = QueueFamilyIndex {
            graphic: 0,
            present: 0,
        };

        assert_eq!(vec![0], index.to_vec());
        assert_eq!(vec![0], <QueueFamilyIndex as Into<Vec<u32>>>::into(index));

        let index = QueueFamilyIndex {
            graphic: 0,
            present: 1,
        };

        assert_eq!(vec![0, 1], index.to_vec());
        assert_eq!(
            vec![0, 1],
            <QueueFamilyIndex as Into<Vec<u32>>>::into(index)
        );
    }
}
