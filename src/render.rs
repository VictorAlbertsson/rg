use crate::window::WindowState;

use std::ffi::{CStr, CString};
use std::borrow::Cow;

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
use ash::extensions::{ext::DebugUtils, khr::{Surface, Swapchain}};
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::{vk, Device, Entry, Instance};
use log::info;

#[derive(Debug, Clone, Copy)]
pub struct Renderer {
    //surface: vk::SurfaceKHR,
}

impl Renderer {
    pub fn new(title: String) -> Self {
	unsafe {
	    let window_state = WindowState::new();
	    let entry = Entry::new().unwrap();
	    let name  = CString::new(title).unwrap();
	    let engine = CString::new("RG").unwrap();
	    let layer_names = [CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
	    let raw_layers: Vec<*const i8> = layer_names
		.iter()
		.map(|raw_name| raw_name.as_ptr())
		.collect();
	    let raw_extensions = extension_names();
	    let appinfo = vk::ApplicationInfo::builder()
		.application_name(&name)
		.application_version(0)
		.engine_name(&engine)
		.engine_version(0)
		.api_version(vk::make_version(1, 0, 0));
	    let create_info = vk::InstanceCreateInfo::builder()
		.application_info(&appinfo)
		.enabled_layer_names(&raw_layers)
		.enabled_extension_names(&raw_extensions);
	    // Beginning of actually unsafe part
	    let instance: Instance = entry
		.create_instance(&create_info, None)
		.expect("Instance creation error");
	    // End of actually unsafe part
	    let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
		.message_severity(
		    vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
			| vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
			| vk::DebugUtilsMessageSeverityFlagsEXT::INFO
		)
		.message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
		.pfn_user_callback(Some(vulkan_debug_callback));
	    let debug_utils_loader = DebugUtils::new(&entry, &instance);
	    let debug_calback = debug_utils_loader
		.create_debug_utils_messenger(&debug_info, None)
		.unwrap();
	    let surface = create_surface(&entry, &instance, &window).unwrap();
	    let pdevices = instance
		.enumerate_physical_devices()
		.expect("Physical device error.");
	    let surface_loader = Surface::new(&entry, &instance);
	    let (pdevice, queue_family_index) = pdevices
		.iter()
		.map(|pdevice| {
		    instance
			.get_physical_device_queue_family_properties(*pdevice)
			.iter()
			.enumerate()
			.filter_map(|(index, ref info)| {
			    let supports_graphic_and_surface =
				info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
				&& surface_loader
				.get_physical_device_surface_support(*pdevice, index as u32, surface)
				.unwrap();
			    if supports_graphic_and_surface {
				Some((*pdevice, index))
			    } else {
				None
			    }
			})
			.next()
		})
		.filter_map(|v| v)
		.next()
		.expect("Couldn't find suitable device.");
	    let queue_family_index = queue_family_index as u32;
	    let device_extension_names_raw = [Swapchain::name().as_ptr()];
	    let features = vk::PhysicalDeviceFeatures {
		shader_clip_distance: 1,
		..Default::default(),
	    };
	    let priorities = [1.0];
	    let queue_info = [vk::DeviceQueueCreateInfo::builder()
			      .queue_family_index(queue_family_index)
			      .queue_priorities(&priorities)
			      .build()];
	    let device_create_info = vk::DeviceCreateInfo::builder()
		.queue_create_infos(&queue_info)
		.enabled_extension_names(&device_extension_names_raw)
		.enabled_features(&features);
	    let device: Device = instance
		.create_device(pdevice, &device_create_info, None)
		.unwrap();
	    let present_queue = device.get_device_queue(queue_family_index as u32, 0);
	    let surface_formats = surface_loader
		.get_physical_device_surface_formats(pdevice, surface)
		.unwrap();
	    let surface_format = surface_formats
		.iter()
		.map(|sfmt| match sfmt.format {
		    vk::Format::UNDEFINED => vk::SurfaceFormatKHR {
			format: vk::Format::B8G8R8_UNORM,
			color_space: sfmt.color_space,
		    },
		    _ => *sfmt,
		})
		.next()
		.expect("Unable to find suitable surface format.");
	    let surface_capabilities = surface_loader
		.get_physical_device_surface_capabilities(pdevice, surface)
		.unwrap();

	}
	Self {

	}
    }
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
unsafe fn create_surface<E, I>(entry: &E, instance: &I, window: &winit::window::Window)
			       -> Result<vk::SurfaceKHR, vk::Result>
where E: EntryV1_0, I: InstanceV1_0 {
    use winit::platform::unix::WindowExtUnix;
    let x11_display = window.xlib_display().unwrap();
    let x11_window  = window.xlib_window().unwrap();
    let x11_create_info = vk::XlibSurfaceCreateInfoKHR::builder()
	.window(x11_window)
	.dpy(x11_display as *mut vk::Display);
    let xlib_surface_loader = XlibSurface::new(entry, instance);
    xlib_surface_loader.create_xlib_surface(&x11_create_info, None)
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub(crate) fn extension_names() -> Vec<*const i8> {
    vec![Surface::name().as_ptr(),
	 XlibSurface::name().as_ptr(),
	 DebugUtils::name().as_ptr()]
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void)
    -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;
    let message_id_name = if callback_data.p_message_id_name.is_null() {
	Cow::from("")
    } else {
	CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };
    let message = if callback_data.p_message.is_null() {
	Cow::from("")
    } else {
	CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };
    info!(
	"{:?}:\n{:?} [{} ({})] : {}\n",
	message_severity,
	message_type,
	message_id_name,
	&message_id_number.to_string(),
	message);
    vk::FALSE
}
