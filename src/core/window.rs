use crate::core::{config, x};
use x11::xlib;

pub type WindowID = u64;
pub type WindowAttributes = xlib::XWindowAttributes;
pub type WindowChanges = xlib::XWindowChanges;

pub struct Window<'a> {
    // Open display
    display: &'a x::Display,

    id: WindowID,
    attrs: WindowAttributes,
    frame: WindowID,
}

#[allow(dead_code)]
impl<'a> Window<'a> {
    pub fn new(display: &'a x::Display, id: WindowID) -> Result<Window, String> {
        let attrs = display.get_window_attributes(id)?;

        let frame = display.create_simple_window(
            display.root(),
            attrs.x,
            attrs.y,
            attrs.width as u32,
            attrs.height as u32,
            config::BORDER_WIDTH,
            config::BORDER_COLOR,
            config::BACKGROUND,
        );
        display.select_input(frame);
        display.add_to_save_set(id);
        display.reparent_window(id, frame);
        display.map_window(frame);

        Ok(Window {
            id,
            attrs,
            frame,
            display,
        })
    }

    pub fn id(&self) -> WindowID {
        self.id
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.attrs.x = x;
        self.attrs.y = y;
        self.display.move_window(self.frame, x, y);
        // self.display.move_window(self.id, x, y);
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        self.attrs.width = w as i32;
        self.attrs.height = h as i32;
        self.display.resize_window(self.frame, w, h);
        self.display.resize_window(self.id, w, h);
    }

    pub fn map(&self) {
        self.display.map_window(self.id);
    }

    pub fn reparent(&self, parent: WindowID) {
        self.display.reparent_window(self.id, parent);
    }
}
