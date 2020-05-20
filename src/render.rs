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
	}
	Self {

	}
    }
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
unsafe fn create_surface<E, I>(entry: &E, instance: &I, window: &winit::window::Window)
			       -> Result<vk::SurfaceKHR, vk::Result>
where E: EntryV1_0,
      I: InstanceV1_0 {
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
