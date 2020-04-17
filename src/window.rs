use crate::events::Event;

pub struct WindowHandler {
    pub window: winit::window::Window,
    pub events: winit::event_loop::EventLoop<()>,
    pub hooks: Vec<Box<dyn Event>>,
}

impl WindowHandler {
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
	    hooks: Vec::new(),
	})
    }
    pub fn add_hook<E>(&mut self, hook: E) where
	E: Event + 'static {
	self.hooks.push(Box::new(hook));
    }
    pub fn run(self) { // Consumes `self`
	self.events.run(move |event, _, _s| {
	    match event {
		winit::event::Event::WindowEvent {
		    event: winit::event::WindowEvent::Resized(psize),
		    ..
		} => {
		    for hook in &self.hooks {
			hook.on_resize(psize.width, psize.height);
		    }
		},
		_ => (),
	    }
	});
    }
}

impl Default for WindowHandler {
    fn default() -> Self {
	Self::new("RG Engine",
		  winit::dpi::LogicalSize {
		      width: 1280.0,
		      height: 720.0,
		  })
	    .expect("Could not create the window!")
    }
}
