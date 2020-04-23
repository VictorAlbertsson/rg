use std::sync::mpsc::Receiver;
use glfw::Context;
use crate::events::Events;

pub fn new_window(glfw: &glfw::Glfw) -> (glfw::Window, Receiver<(f64, glfw::WindowEvent)>) {
    let (mut window, events) = glfw.create_window(300, 300, "Hello", glfw::WindowMode::Windowed)
	.expect("Failed to create GLFW window!");
    window.make_current();
    window.set_key_polling(true);
    (window, events)
}

pub fn start_window(mut glfw: glfw::Glfw, mut window: glfw::Window, events: Receiver<(f64, glfw::WindowEvent)>) {
    while !window.should_close() {
	window.swap_buffers();
	glfw.poll_events();
	for (_, event) in glfw::flush_messages(&events) {
	    // DEVELOPMENT ONLY, VERY TEMPORARY!
	    println!("{:?}", event);
	    match event {
		glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>  {
		    window.on_close();
		},
		_ => (),
	    }
	}
    }
}
