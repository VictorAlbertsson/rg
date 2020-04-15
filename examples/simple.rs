use simple_logger;
use log::{info};

use rg::engine::events::Event;

/// Simple example
/// At the moment it is just a sandbox
fn main() {
    simple_logger::init().unwrap();

    let event = Event::Keypress {
	keycode: 'a',
	repeats: 0,
    };
    event.dispatch(testf);
}

// TODO: Make a macro to remove match biolerplate
fn testf(event: &Event) -> bool {
    match event {
	Event::Keypress {
	    keycode: _,
	    repeats: _,
	} => info!("Test worked!"),
	_ => info!("Test didn't work!"),
    }

    true
}
