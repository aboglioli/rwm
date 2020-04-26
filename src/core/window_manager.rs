use std::collections::HashMap;

use crate::core::{
    config,
    event::{self, Event},
    window, x,
};

pub struct WindowManager {
    display: x::Display,
    windows: HashMap<window::WindowID, window::Window>,
    cleaned: bool,
}

impl WindowManager {
    pub fn new() -> Result<WindowManager, String> {
        Ok(WindowManager {
            display: x::Display::open()?,
            windows: HashMap::new(),
            cleaned: false,
        })
    }

    pub fn scan(&mut self) -> Result<(), String> {
        let (_, _, window_ids) = self.display.query_tree(self.display.root())?;

        for win_id in window_ids {
            let attrs = self.display.get_window_attributes(win_id)?;
            let mut win = window::Window::new(win_id);
            win.set_attributes(attrs);
            self.windows.insert(win_id, win);
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
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

    pub fn create_frame(&self, w: window::WindowID) -> Result<window::WindowID, String> {
        let attrs = self.display.get_window_attributes(w)?;
        let frame = self.display.create_simple_window(
            self.display.root(),
            attrs.x,
            attrs.y,
            attrs.width as u32,
            attrs.height as u32,
            config::BORDER_WIDTH,
            config::BORDER_COLOR,
            config::BACKGROUND,
        );

        self.display.select_input(frame);
        self.display.add_to_save_set(w);
        self.display.reparent_window(w, frame);
        self.display.map_window(frame);

        Ok(frame)
    }

    pub fn on_create(&self, create: event::CreateWindowEvent) {}

    pub fn on_configure_request(&self, req: event::ConfigureRequestEvent) {
        let mut changes = window::WindowChanges {
            x: req.x,
            y: req.y,
            width: req.width,
            height: req.height,
            border_width: req.border_width,
            sibling: req.above,
            stack_mode: req.detail,
        };
        self.display
            .configure_window(req.window, req.value_mask, &mut changes);
    }

    pub fn on_map_request(&mut self, map_req: event::MapRequestEvent) {
        if let Ok(frame) = self.create_frame(map_req.window) {
            println!("- Frame created: {}", frame);
            self.display.map_window(map_req.window);

            if let Some(win) = self.windows.get_mut(&map_req.window) {
                win.set_frame(frame);
            } else {
                let mut win = window::Window::new(map_req.window);
                win.set_frame(frame);
                if let Ok(attrs) = self.display.get_window_attributes(win.id) {
                    win.set_attributes(attrs);
                }
                self.windows.insert(win.id, win);
            };
        }
        println!("- Mapped: {}", map_req.window);
    }

    pub fn on_destroy(&self, destroy: event::DestroyWindowEvent) {}

    pub fn on_reparent(&self, reparent: event::ReparentEvent) {}

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
