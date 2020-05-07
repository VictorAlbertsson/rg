use log::info;
use winit::window::Window;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

// Beacuse Rust is dumb and can't borrow individual struct fields!
fn p_run(events: EventLoop<()>, window: Window, mut settings: WindowSettings) {
    // TODO: Support multiple windows
    events.run(move |event, _, control_flow| {
	// Perhaps more logic should bo moved into the
	// if-statement.
	if settings.poll_events {
	    *control_flow = ControlFlow::Poll;
	} else {
	    *control_flow = ControlFlow::Wait;
	}
	// TODO: Remove debug printing!
	match event {
	    Event::WindowEvent {
		event: WindowEvent::CloseRequested,
		..
	    } => settings.running = false,
	    Event::MainEventsCleared => {
		// Application logic goes here.
		window.request_redraw();
	    },
	    Event::RedrawRequested(_) => {
		// Render logic goes here.
	    },
	    _ => (),
	}
	if !settings.running {
	    info!("The close button was pressed; closing...");
	    *control_flow = ControlFlow::Exit;
	}
    })
}

#[derive(Debug, Clone)]
pub struct WindowSettings {
    pub running: bool,
    pub poll_events: bool,
    pub frame_size: Option<(f64, f64)>,
    pub mouse_position: Option<(f64, f64)>,
}

impl Default for WindowSettings {
    fn default() -> Self {
	Self {
	    running: true,
	    poll_events: true,
	    frame_size: Some((0.0, 0.0)),
	    mouse_position: Some((0.0, 0.0)),
	}
    }
}

pub struct WindowState {
    pub settings: WindowSettings,
    pub window: Window,
    pub events: EventLoop<()>,
}

impl Default for WindowState {
    fn default() -> Self {
	let settings = WindowSettings::default();
	let events = EventLoop::new();
	// TODO: Unwrap is non-idiomatic, use expect instead.
	let window = Window::new(&events).ok().unwrap();
	Self {
	    settings,
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
	p_run(self.events, self.window, self.settings);
    }
}
