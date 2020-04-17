pub trait Event {
    fn on_user_event(&self);
    fn on_close(&self);
    fn on_resize(&self, x: u32, y: u32);
    fn on_focus(&self);
    fn on_focus_lost(&self);
    fn on_moved(&self);
    fn on_tick(&self);
    fn on_update(&self);
    fn on_render(&self);
    fn on_keypress(&self, key: char, repeats: u32);
    fn on_keyrelease(&self, key: char);
    fn on_mouse_moved(&self, x: f64, y: f64);
    fn on_mouse_scrolled(&self, x_offset: f64, y_offset: f64);
    fn on_mouse_buttonpress(&self, button: u32);
    fn on_mouse_buttonrelease(&self, button: u32);
}

impl Event for winit::window::Window {
    fn on_user_event(&self) {

    }
    fn on_close(&self) {

    }
    fn on_resize(&self, x: u32, y: u32) {

    }
    fn on_focus(&self) {

    }
    fn on_focus_lost(&self) {

    }
    fn on_moved(&self) {

    }
    fn on_tick(&self) {

    }
    fn on_update(&self) {

    }
    fn on_render(&self) {

    }
    fn on_keypress(&self, key: char, repeats: u32) {

    }
    fn on_keyrelease(&self, key: char) {

    }
    fn on_mouse_moved(&self, x: f64, y: f64) {

    }
    fn on_mouse_scrolled(&self, x_offset: f64, y_offset: f64) {

    }
    fn on_mouse_buttonpress(&self, button: u32) {

    }
    fn on_mouse_buttonrelease(&self, button: u32) {

    }
}
