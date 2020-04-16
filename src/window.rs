//use crate::events::{Event, WinitEventHandler};

pub struct Window {
    pub window: winit::window::Window,
    pub events: winit::event_loop::EventLoop<()>,
}

impl Window {
    pub fn new<T, U>(title: T, size: U) -> Result<Self, winit::error::OsError> where
	T: Into<String>,
	U: Into<winit::dpi::Size> {
	let events = winit::event_loop::EventLoop::new();
	let output = winit::window::WindowBuilder::new()
	    .with_title(title)
	    .with_inner_size(size)
	    .build(&events);
	output.map(|window| Self {
	    window,
	    events,
	})
    }
    pub fn run(self) { // Consumes `self`
//	let mut winit_handler = WinitEventHandler::new();
	//handler.register();
	self.events.run(move |_e, _, _s| {
//	    handler.notify(&temp);
	});
    }
}

impl Default for Window {
    fn default() -> Self {
	Self::new("RG Engine",
		  winit::dpi::LogicalSize {
		      width: 1280.0,
		      height: 720.0,
		  })
	    .expect("Could not create the window!")
    }
}
