use crate::events::{Event, WindowEventHandler, WindowEvent};

pub struct Window {
    pub window: winit::window::Window,
    pub events: winit::event_loop::EventLoop<()>,
}

impl<'x> Window {
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
	let mut handler = WindowEventHandler::new(self.window);
	handler.register(on_resize);
	self.events.run(move |event, _, _s| {
	    match event {
		winit::event::Event::WindowEvent {
		    event: winit::event::WindowEvent::Resized(psize),
		    ..
		} => handler.notify(WindowEvent::Resize(psize.width, psize.height)),
		_ => (),
	    }
	});
    }
}

pub fn on_resize<'a>(event: &'a WindowEvent, input: &Window) -> &'a WindowEvent {
    match event {
	WindowEvent::Resize(x, y) => {
	    input.window.request_redraw();
	    &event
	},
	_ => &event,
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
