use std::any::Any;
use crate::window::Window;
use std::cell::RefCell;

/// Requires that the user spacifies a static lifetime
pub trait Event<T> {
    // This might fail
    fn register<D>(&mut self, hook: Hook<T, D>);
    fn notify(&self, message: T);
}

pub type Hook<T, D> = fn(&T, D) -> &T;

/// Generic event handler
/// If you need to store more data you can make your own handler
pub struct EventHandler<T, D> {
    // Are vectors of unsized elements allowed?
    // Hooks are unsized.
    hooks: Vec<Hook<T, D>>,
}

impl<T, D> EventHandler<T, D> {
    pub fn new() -> Self {
	Self {
	    hooks: vec![]
	}
    }
}

/// Generic event implementation
impl<T, D> Event<T> for EventHandler<T, D> {
    fn register<C>(&mut self, hook: Hook<T, C>) {
	self.hooks.push(hook);
    }
    fn notify(&self, event: T) {
	for callback in self.hooks {
//	    (callback.into_inner())(&event);
	}
    }
}

pub struct WindowEventHandler {
    pub window: winit::window::Window,
    pub hooks: Vec<Hook<WindowEvent, winit::window::Window>>,
}

impl WindowEventHandler {
    pub fn new(w: winit::window::Window) -> Self {
	Self {
	    window: w,
	    hooks: vec![],
	}
    }
}

impl Event<WindowEvent> for WindowEventHandler {
    fn register<winit::window::Window>(&mut self, hook: Hook<WindowEvent, winit::window::Window>) {
	self.hooks.push(hook);
    }
    fn notify(&self, event: WindowEvent) {
	for callback in self.hooks {
	    callback(&event, self.window);
	}
    }
}

pub enum WindowEvent {
    Close,
    Resize(u32, u32),
    Focus,
    LostFocus,
    Moved,
    Tick,
    Update,
    Render,
}

pub enum InputEvent {
    Keypress(char, u32),
    Keyrelease(char),
    MouseMoved(f64, f64),
    MouseScrolled(f64, f64),
    MouseButtonpress(u32),
    MouseButtonrelease(u32),
}
