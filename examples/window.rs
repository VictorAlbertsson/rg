use rg::window::WindowState;

fn main() {
    simple_logger::init().unwrap();
    let ws = WindowState::new();
    ws.run();
}
