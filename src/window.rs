use winit::window::Window;
use winit::event::{Event, WindowEvent, VirtualKeyCode};
use winit::event_loop::{EventLoop, ControlFlow};
use log::info;

pub struct WindowState {
    pub window: Window,
    pub events: EventLoop<()>,
}

impl Default for WindowState {
    fn default() -> Self {
	let events = EventLoop::new();
	// TODO: Unwrap is non-idiomatic, use expect instead.
	let window = Window::new(&events).ok().unwrap();
	info!("Window created.");
	Self {
	    events,
	    window,
	}
    }
}

impl WindowState {
    pub fn new() -> Self {
	Self::default()
    }
    pub fn run(self) {
	// Who else thinks this is really dumb?
	p_run(self.events, self.window);
    }
}

// Beacuse Rust is dumb and can't borrow individual struct fields!
fn p_run(events: EventLoop<()>, window: Window) {
    // TODO: Support multiple windows
    events.run(move |event, _, control_flow| {
	match event {
	    Event::WindowEvent { event, .. } => match event {
		WindowEvent::KeyboardInput { input, .. } => {
		    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
			*control_flow = ControlFlow::Exit;
		    } else {
			*control_flow = ControlFlow::Poll;
		    }
		}
		WindowEvent::CloseRequested => {
		    *control_flow = ControlFlow::Exit;
		}
		_ => (),
	    }
	    Event::MainEventsCleared => {
		// Application logic goes here.
		window.request_redraw();
	    }
	    Event::RedrawRequested(_) => {
		// Render logic goes here.
	    }
	    _ => (),
	}
    })
}
