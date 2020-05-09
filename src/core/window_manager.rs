use std::rc::Rc;

use crate::core::{
    event::{self, Event},
    layout, node,
    node::Node,
    window, x,
};

pub struct WindowManager {
    display: Rc<x::Display>,

    windows: Vec<Box<dyn node::Node>>,

    layouts: Vec<Box<dyn layout::Layout>>,
    selected_layout: usize,
}

impl WindowManager {
    pub fn new(display: x::Display) -> WindowManager {
        WindowManager {
            display: Rc::new(display),

            windows: Vec::new(),

            layouts: vec![
                Box::new(layout::ColumnLayout(800, 600)),
                // Box::new(layout::RowLayout(800, 600)),
            ],
            selected_layout: 0,
        }
    }

    pub fn scan(&mut self) -> Result<usize, String> {
        let (_, _, window_ids) = self.display.query_tree(self.display.root())?;
        let len = window_ids.len();

        let mut focused_window = false;
        for win_id in window_ids {
            let attrs = self.display.get_window_attributes(win_id)?;

            if attrs.override_redirect > 0 || attrs.map_state != x::IS_VIEWABLE {
                continue;
            }

            let mut win = window::Window::new(&self.display, win_id, attrs);
            if !focused_window {
                win.focus();
                focused_window = true;
            }

            self.windows.push(Box::new(win));
        }

        self.apply_selected_layout();

        Ok(len)
    }

    pub fn grab_events(&self) {}

    pub fn run(&mut self) -> Result<(), String> {
        self.grab_events();

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
        self.layouts[self.selected_layout].apply(&mut self.windows.iter_mut());
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

        if let Some(_) = self.windows.iter_mut().find(|win| win.id() == req.window) {
            // self.display
            //     .configure_window(win.frame(), req.value_mask, &mut changes);
        }

        self.display
            .configure_window(req.window, req.value_mask, &mut changes);
    }

    fn on_map_request(&mut self, req: event::MapRequestEvent) {
        let win_id = req.window;

        if let Some(win) = self.windows.iter().find(|win| win.id() == win_id) {
            win.map();
            return;
        }

        if let Ok(attrs) = self.display.get_window_attributes(win_id) {
            if attrs.override_redirect > 0 || attrs.map_state != x::IS_VIEWABLE {
                let win = window::Window::new(&self.display, win_id, attrs);
                win.map();
                self.windows.push(Box::new(win));
            }
        }

        self.apply_selected_layout();
    }

    fn on_unmap_notify(&mut self, req: event::UnmapEvent) {
        if req.event == self.display.root() {
            return;
        }

        let win_id = req.window;

        if let Some((i, _)) = self
            .windows
            .iter_mut()
            .enumerate()
            .find(|(_, win)| win.id() == win_id)
        {
            self.windows.remove(i);
        }

        self.apply_selected_layout();
    }
}
