extern crate winapi;
#[macro_use]
pub mod macros;
pub mod application;
pub mod events;
pub mod render_engine;
pub mod window;

fn main() {
    let mut app = application::Application::new();
    app.run();
}
