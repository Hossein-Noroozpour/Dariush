extern crate winapi;

pub mod application;
pub mod events;
pub mod window;

fn main() {
    let mut app = application::Application::new();
    app.run();
}
