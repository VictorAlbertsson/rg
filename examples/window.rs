use rg::window::{new_window, start_window};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (window, events) = new_window(&glfw);
    start_window(glfw, window, events);
}
