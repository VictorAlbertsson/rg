use std::sync::mpsc::Receiver;
use glfw::Context;
use crate::events::Events;
use crate::events::WindowEvents;

pub struct Window {
    pub window: glfw::Window,
    pub context: glfw::Glfw,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new() -> Self {
	let context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
	let (window, events) = context.create_window(300, 300, "Test", glfw::WindowMode::Windowed).unwrap();
	let mut w = Self {
	    window,
	    context,
	    events,
	};
	// Calls the on_create callbacks
	w.on_create();
	// Returns the Self struct
	w
    }
    pub fn run(mut self) {
	// One window only a.t.m
	while !self.window.should_close() {
	    self.on_tick();
	    self.on_update();
	    self.on_render();
	}
    }
}
