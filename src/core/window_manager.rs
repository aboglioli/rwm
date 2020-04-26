use crate::core::{
    event::{self, Event},
    window, x,
};
use x11::xlib;

pub struct WindowManager {
    display: x::Display,
    windows: Vec<window::Window>,
    cleaned: bool,
}

impl WindowManager {
    pub fn new() -> Result<WindowManager, String> {
        Ok(WindowManager {
            display: x::Display::open()?,
            windows: Vec::new(),
            cleaned: false,
        })
    }

    pub fn scan(&self) -> Result<(), String> {
        let windows = self.display.get_windows()?;
        Ok(())
    }

    pub fn run(&self) -> Result<(), String> {
        self.display.grab_button(
            1,
            0,
            self.display.root(),
            1,
            event::BUTTON_PRESS_MASK | event::BUTTON_RELEASE_MASK | event::POINTER_MOTION_MASK,
            event::GRAB_MODE_ASYNC,
            event::GRAB_MODE_ASYNC,
            0,
            0,
        );

        loop {
            match self.display.next_event() {
                Event::Create(create) => self.on_create(create),
                Event::ConfigureRequest(configure_req) => self.on_configure_request(configure_req),
                Event::MapRequest(map_req) => self.on_map_request(map_req),
                Event::Destroy(destroy) => self.on_destroy(destroy),
                Event::Reparent(reparent) => self.on_reparent(reparent),

                Event::ButtonPress(button) => {}
                Event::Motion(button, motion) => {}
                _ => (),
            }
        }
    }

    pub fn on_create(&self, create: event::CreateWindowEvent) {
        println!("Create: {:?}", create);
    }

    pub fn on_configure_request(&self, configure_req: event::ConfigureRequestEvent) {
        println!("Configure: {:?}", configure_req);
    }

    pub fn on_map_request(&self, map_req: event::MapRequestEvent) {
        println!("Map: {:?}", map_req);
    }

    pub fn on_destroy(&self, destroy: event::DestroyWindowEvent) {
        println!("Destroy: {:?}", destroy);
    }

    pub fn on_reparent(&self, reparent: event::ReparentEvent) {
        println!("Reparent: {:?}", reparent);
    }

    pub fn cleanup(&mut self) {
        if !self.cleaned {
            // Clean up
            // TODO

            self.cleaned = true;
        }
    }
}

impl Drop for WindowManager {
    fn drop(&mut self) {
        self.cleanup();
    }
}
