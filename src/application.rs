use super::window::Window;
use super::events::Event;
use std::mem::zeroed;
use std::process::exit;
use std::sync::Mutex;
use super::render_engine::RenderEngine;

pub struct Application {
    pub window: Window,
    pub render_engine: RenderEngine,
    event_handler_locker: Mutex<()>,
}

impl Application {
    pub fn new() -> Self {
        let mut app = Application {
            window: unsafe { zeroed() },
            render_engine: RenderEngine::new(),
            event_handler_locker: Mutex::new(()),
        };
        Window::new(&mut app);
        app
    }

    pub fn run(&mut self) {
        self.window.main_loop();
    }

    pub fn handle(&mut self, e: Event) {
        match e {
            Event::Quit => exit(0),
            _ => {}
        }
        let _guard = self.event_handler_locker.lock().unwrap();
        match e {
            Event::WindowShowedUp => self.render_engine.initialize(&self.window),
            _ => {
                eprintln!("Unhandled app event.");
            }
        }
    }
}
