//use winit::event_loop::EventLoopWindowTarget;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
    dpi::{Size, LogicalSize},
};
use std::sync::Arc;
use vulkano::device::{
    Device,
    DeviceExtensions,
    Queue,
    Features,
};
use vulkano::instance::{
    Instance,
    InstanceExtensions,
    ApplicationInfo,
    Version,
    layers_list,
    PhysicalDevice,
};

use vulkano::instance::debug::{DebugCallback, MessageType, MessageSeverity};

const VALIDATION_LAYERS: &[&str] = &[
    "VK_LAYER_KHRONOS_validation"
];

// TODO: Submit pull request to fix error in tutorial
// const VALIDATION_LAYERS: &[&str] = &[
//     "VK_LAYER_LUNARG_standard_validation"
// ];
// OUTPUT: Validation layers requested, but not available!

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

fn main() {
    RGBuilder::new()
	.with_name("Demo App")
	.with_size(LogicalSize::new(100, 100))
	.start();
}

pub struct RGBuilder {
    name: &'static str,
    size: Size,
}

impl RGBuilder {
    pub fn new() -> Self {
	Self {
	    name: "",
	    size: Size::new(LogicalSize::new(0, 0)),
	}
    }
    pub fn with_name(mut self, name: &'static str) -> Self {
	self.name = name;
	self
    }
    pub fn with_size<S: Into<Size>>(mut self, size: S) -> Self {
	self.size = size.into();
	self
    }
    pub fn start(self) -> ! {
	let (events, rg) = RG::initialize(self);
	events.run(rg)
    }
}

// TODO: Implement `EventManager` as a trait
pub struct EventManager {
    events: EventLoop<()>
}

impl EventManager {
    pub fn new(events: EventLoop<()>) -> Self {
	Self { events }
    }

    pub fn run(self, rg: RG) -> ! {
	self.events.run(move |event, _, control_flow| {
	    *control_flow = ControlFlow::Poll;

	    match event {
		Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
		    println!("Termination signal received; Stopping.");
		    *control_flow = ControlFlow::Exit;
		},
		Event::MainEventsCleared => {
		    rg.window.request_redraw();
		},
		Event::RedrawRequested(_) => {
		    // Render
		},
		_ => (),
	    }
	});
    }
}

struct QueueFamilyIndices {
    graphics_family: i32,
}

impl QueueFamilyIndices {
    fn new() -> Self {
	Self { graphics_family: -1 }
    }

    fn is_complete(&self) -> bool {
	self.graphics_family >= 0
    }
}

pub struct RG {
    pub debug_callback: Option<DebugCallback>,
    pub instance: Arc<Instance>,
    pub window: Window,
    pub physical_device_index: usize, // Can't store PhysicalDevice directly (lifetime issues)
    pub device: Arc<Device>,
    pub graphics_queue: Arc<Queue>,
}

impl RG {
    pub fn initialize(settings: RGBuilder) -> (EventManager, RG) {
	let instance = Self::create_instance();
	let debug_callback = Self::setup_debug_callback(&instance);
	let (window, events) = Self::init_window(settings);
	let physical_device_index = Self::pick_physical_device(&instance);
	let (device, graphics_queue) = Self::create_logical_device(&instance, physical_device_index);

	(EventManager::new(events),
	 Self {
	     debug_callback,
	     instance,
	     window,
	     physical_device_index,
	     device,
	     graphics_queue,
	})
    }

    fn create_logical_device(instance: &Arc<Instance>, physical_device_index: usize) -> (Arc<Device>, Arc<Queue>) {
	let physical_device = PhysicalDevice::from_index(&instance, physical_device_index).unwrap();
	let indices = Self::find_queue_families(&physical_device);
	let queue_family = physical_device.queue_families().nth(indices.graphics_family as usize).unwrap();
	let queue_priority = 1.0;

        // NOTE: the (Vulkan) tutorial recommends passing the validation layers as well
        // for legacy reasons (if ENABLE_VALIDATION_LAYERS is true). Vulkano handles that
        // for us internally.
	let (device, mut queues) = Device::new(physical_device,
					       &Features::none(),
					       &DeviceExtensions::none(),
					       [(queue_family, queue_priority)].iter().cloned())
	    .expect("Failed to create logical device!");
	let graphics_queue = queues.next().unwrap();

	(device, graphics_queue)
    }

    fn find_queue_families(device: &PhysicalDevice) -> QueueFamilyIndices {
	let mut indices = QueueFamilyIndices::new();
	for (i, queue_family) in device.queue_families().enumerate() {
	    if queue_family.supports_graphics() {
		indices.graphics_family = i as i32;
	    }

	    if indices.is_complete() {
		break;
	    }
	}

	indices
    }

    fn is_device_suitable(device: &PhysicalDevice) -> bool {
	let indices = Self::find_queue_families(device);
	indices.is_complete()
    }

    fn pick_physical_device(instance: &Arc<Instance>) -> usize {
	PhysicalDevice::enumerate(&instance)
	    .position(|device| Self::is_device_suitable(&device))
	    .expect("Failed to find a suitable GPU!")
    }

    fn init_window(settings: RGBuilder) -> (Window, EventLoop<()>) {
	let events = EventLoop::new();

	let window = WindowBuilder::new()
	    .with_title(settings.name)
	    .with_inner_size(settings.size)
	    .build(&events)
	    .ok()
	    .unwrap();

	(window, events)
    }

    fn create_instance() -> Arc<Instance> {
	if ENABLE_VALIDATION_LAYERS && !Self::check_validation_layer_support() {
	    // TODO Log instead of printing
	    println!("Validation layers requested, but not available!");
	}

	let supported_extensions = InstanceExtensions::supported_by_core()
	    .expect("Failed to retrieve supported extensions");

	// TODO Log instead of printing
	println!("Supported extensions: {:?}", supported_extensions);

	let app_info = ApplicationInfo {
	    application_name: Some("Hello Triangle".into()),
	    application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
	    engine_name: Some("No Engine".into()),
	    engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
	};

	let required_extensions = Self::get_required_extensions();

	if ENABLE_VALIDATION_LAYERS && Self::check_validation_layer_support() {
	    Instance::new(Some(&app_info), &required_extensions, VALIDATION_LAYERS.iter().cloned())
		.expect("Failed to create Vulkan instance")
	} else {
	    Instance::new(Some(&app_info), &required_extensions, None)
		.expect("Failed to create Vulkan instance")
	}
    }

    fn check_validation_layer_support() -> bool {
	let layers: Vec<_> =
	    layers_list()
	    .unwrap()
	    .map(|layer| layer.name().to_owned())
	    .collect();

	VALIDATION_LAYERS.iter()
	    .all(|layer_name| layers.contains(&layer_name.to_string()))
    }

    fn get_required_extensions() -> InstanceExtensions {
	let mut extensions = vulkano_win::required_extensions();

	if ENABLE_VALIDATION_LAYERS {
	    extensions.ext_debug_utils = true;
	}

	extensions
    }

    fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
	if !ENABLE_VALIDATION_LAYERS {
	    return None;
	}

	let msg_severity = MessageSeverity {
	    error: true,
	    warning: true,
	    information: true,
	    verbose: true,
	};

	let msg_types = MessageType {
	    general: true,
	    validation: true,
	    performance: true,
	};

	DebugCallback::new(&instance,
			   msg_severity,
			   msg_types,
			   |msg| println!("Validation layer: {:?}", msg.description))
	    .ok()
    }
}
