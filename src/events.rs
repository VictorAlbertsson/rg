pub trait Events {
    fn on_none();
    fn on_close(&mut self);
    fn on_resize(x: u32, y: u32);
    fn on_focus();
    fn on_focus_lost();
    fn on_moved();
    fn on_tick();
    fn on_update();
    fn on_render();
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

// TODO: Implement trait fully
impl Events for glfw::Window {
    fn on_none() {}
    fn on_close(&mut self) {
	self.set_should_close(true);
    }
    fn on_resize(x: u32, y: u32) {}
    fn on_focus() {}
    fn on_focus_lost() {}
    fn on_moved() {}
    fn on_tick() {}
    fn on_update() {}
    fn on_render() {}
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
