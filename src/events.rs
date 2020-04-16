pub trait Event<T> {
    fn register(&mut self, hook: &'static Hook<T>);
    fn notify(&self, message: &'static T);
}

pub type Hook<T> = dyn Fn(&T);
// Might remove this one
pub type WEvent<'a> = winit::event::Event<'a, ()>;

/// Core window event handler
pub struct WinitEventHandler {
    pub hooks: Vec<Box<Hook<winit::event::Event<'static, ()>>>>
}

impl WinitEventHandler {
    pub fn new() -> Self {
	Self {
	    hooks: vec![]
	}
    }
}

impl Event<winit::event::Event<'_, ()>> for WinitEventHandler {
    fn register(&mut self, hook: &'static Hook<winit::event::Event<'_, ()>>) {
	self.hooks.push(Box::new(hook));
    }
    fn notify(&self, event: &'static winit::event::Event<'_, ()>) {
	for callback in &self.hooks {
	    callback(event);
	}
    }
}

// TODO: WindowEvent Callbacks

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
