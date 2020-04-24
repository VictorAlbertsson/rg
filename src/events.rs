use crate::window::Window;

use glfw::Context;

pub trait Events {
    fn on_create(&mut self);
    fn on_delete(&mut self);
}

pub trait WindowEvents: Events {
    fn on_resize(&mut self, x: u32, y: u32);
    fn on_focus();
    fn on_focus_lost();
    fn on_moved();
    fn on_tick(&mut self);
    fn on_update(&mut self);
    fn on_render(&mut self);
    fn on_key_pressed(keycode: char, repeats: u32);
    fn on_key_released(keycode: char);
    fn on_mouse_moved(x: f64, y: f64);
    fn on_mouse_scrolled(dx: f64, dy: f64);
    fn on_mouse_pressed(button: u32);
    fn on_mouse_released(button: u32);
    fn on_mousemoved(x: f64, y: f64);
    fn on_mousescrolled(dx: f64, dy: f64);
    fn on_mousepress(button: u32);
    fn on_mouserelease(button: u32);
}

impl Events for Window {
    fn on_create(&mut self) {
	self.window.make_current();
	self.window.set_key_polling(true);
    }
    fn on_delete(&mut self) {
	self.window.set_should_close(true);
    }
}

impl WindowEvents for Window {
    fn on_resize(&mut self, x: u32, y: u32) {}
    fn on_focus() {}
    fn on_focus_lost() {}
    fn on_moved() {}
    fn on_tick(&mut self) {}
    fn on_update(&mut self) {
	self.window.swap_buffers();
	self.context.poll_events();
	for (_, event) in glfw::flush_messages(&self.events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
    fn on_render(&mut self) {}
    fn on_key_pressed(keycode: char, repeats: u32) {}
    fn on_key_released(keycode: char) {}
    fn on_mouse_moved(x: f64, y: f64) {}
    fn on_mouse_scrolled(dx: f64, dy: f64) {}
    fn on_mouse_pressed(button: u32) {}
    fn on_mouse_released(button: u32) {}
    fn on_mousemoved(x: f64, y: f64) {}
    fn on_mousescrolled(dx: f64, dy: f64) {}
    fn on_mousepress(button: u32) {}
    fn on_mouserelease(button: u32) {}
}
