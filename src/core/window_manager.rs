use crate::core::{
    event::{self, Event},
    layout, window, x,
};
use std::collections::HashMap;

pub struct WindowManager<'a> {
    display: &'a x::Display,

    windows: HashMap<window::WindowID, window::Window<'a>>,
    selected_window: Option<usize>,

    layouts: Vec<Box<dyn layout::Layout>>,
    selected_layout: usize,
}

impl<'a> WindowManager<'a> {
    pub fn new(display: &x::Display) -> WindowManager {
        WindowManager {
            display,

            windows: HashMap::new(),
            selected_window: None,

            layouts: vec![
                Box::new(layout::ColumnLayout(800, 600)),
                Box::new(layout::RowLayout(800, 600)),
            ],
            selected_layout: 1,
        }
    }

    pub fn scan(&mut self) -> Result<usize, String> {
        let (_, _, window_ids) = self.display.query_tree(self.display.root())?;
        let len = window_ids.len();

        for win_id in window_ids {
            if let Ok(win) = window::Window::new(self.display, win_id) {
                self.windows.insert(win_id, win);
            }
        }

        self.layouts[self.selected_layout].apply(Box::new(self.windows.values_mut()));

        Ok(len)
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
                Event::ConfigureRequest(configure_req) => self.on_configure_request(configure_req),
                Event::MapRequest(map_req) => self.on_map_request(map_req),
                _ => (),
            }
        }
    }

    pub fn on_configure_request(&mut self, req: event::ConfigureRequestEvent) {
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
        let win_id = map_req.window;

        if let Some(win) = self.windows.get_mut(&win_id) {
            win.map();
            return;
        }

        if let Ok(win) = window::Window::new(self.display, win_id) {
            self.windows.insert(win_id, win);
        }
    }
}

// impl<'a> Drop for WindowManager<'a> {
//     fn drop(&mut self) {}
// }
