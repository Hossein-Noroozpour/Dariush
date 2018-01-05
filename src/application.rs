use super::window::Window;
use std::mem::zeroed;

pub struct Application {
    pub is_running: bool,
    pub window: Window,
}

impl Application {
    pub fn new() -> Self {
        let mut app: Application = unsafe { zeroed() };
        app.is_running = true;
        app
    }

    pub fn run(&mut self) {
        Window::new(self);
    }

    pub fn handle(&mut self) {}
}
