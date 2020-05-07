use crate::core::{
    event::{self, Event},
    layout, window, x,
};
use std::collections::HashMap;

pub struct WindowManager<'a> {
    display: &'a x::Display,

    windows: HashMap<window::WindowID, window::Window<'a>>,
    focused_window: Option<usize>,

    layouts: Vec<Box<dyn layout::Layout>>,
    selected_layout: usize,
}

impl<'a> WindowManager<'a> {
    pub fn new(display: &x::Display) -> WindowManager {
        WindowManager {
            display,

            windows: HashMap::new(),
            focused_window: None,

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
            let attrs = self.display.get_window_attributes(win_id)?;

            if attrs.override_redirect > 0 || attrs.map_state != x::IS_VIEWABLE {
                continue;
            }

            let mut win = window::Window::new(self.display, win_id, attrs);
            win.frame();
            self.windows.insert(win_id, win);
        }

        self.apply_selected_layout();

        Ok(len)
    }

    pub fn grab_keys(&self) {}

    pub fn run(&mut self) -> Result<(), String> {
        self.grab_keys();

        loop {
            match self.display.next_event() {
                Event::ConfigureRequest(configure_req) => self.on_configure_request(configure_req),
                Event::MapRequest(req) => self.on_map_request(req),
                Event::UnmapNotify(unmap_req) => self.on_unmap_notify(unmap_req),
                _ => (),
            }
        }
    }

    fn apply_selected_layout(&mut self) {
        self.layouts[self.selected_layout].apply(Box::new(self.windows.values_mut()));
    }

    fn on_configure_request(&mut self, req: event::ConfigureRequestEvent) {
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

    fn on_map_request(&mut self, req: event::MapRequestEvent) {
        let win_id = req.window;

        if let Some(win) = self.windows.get_mut(&win_id) {
            win.map();
            return;
        }

        if let Ok(attrs) = self.display.get_window_attributes(win_id) {
            if attrs.override_redirect > 0 || attrs.map_state != x::IS_VIEWABLE {
                let mut win = window::Window::new(self.display, win_id, attrs);
                win.frame();
                win.map();
                self.windows.insert(win_id, win);
            }
        }

        self.apply_selected_layout();
    }

    fn on_unmap_notify(&mut self, req: event::UnmapEvent) {
        if req.event == self.display.root() {
            return;
        }

        let win_id = req.window;
        if let Some(mut win) = self.windows.remove(&win_id) {
            win.unframe();
        }

        self.apply_selected_layout();
    }
}
