use rg::events::{Event, EventHandler};
use rg::events::WindowEvent;

fn main() {
    fn test1(x: &WindowEvent) {
	match x {
	    WindowEvent::Close => println!("test1: Closing window!"),
	    _ => (),
	}
    }
    fn test2(x: &WindowEvent) {
	match x {
	    WindowEvent::Close => println!("test2: Closing window!"),
	    _ => (),
	}
    }

    let mut handler = EventHandler::new();
    handler.register(&test1);
    handler.register(&test2);
    handler.notify(&WindowEvent::Close);
}
