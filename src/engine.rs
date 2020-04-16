use crate::window::Window;

use std::vec;

pub struct Application {
    pub windows: Vec<Window>,
}

impl Application {
    pub fn new<T, U>(title: T, size: U) -> Self where
	T: Into<String>,
	U: Into<winit::dpi::Size> {
	// Maybe this should be in a Default implementation
	Self {
	    windows: vec![Window::new(title, size)
			  .ok()
			  .expect("Could not create the window!")],
	}
    }
}
