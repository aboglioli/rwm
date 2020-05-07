use crate::core::{config, x};
use x11::xlib;

pub type WindowID = u64;
pub type WindowAttributes = xlib::XWindowAttributes;
pub type WindowChanges = xlib::XWindowChanges;

pub struct Window<'a> {
    // Open display
    display: &'a x::Display,

    id: WindowID,
    x: i32,
    y: i32,
    width: u32,
    height: u32,

    focused: bool,
    marked: bool,

    frame: Option<WindowID>,
}

#[allow(dead_code)]
impl<'a> Window<'a> {
    pub fn new(display: &'a x::Display, id: WindowID, attrs: WindowAttributes) -> Window {
        Window {
            display,
            id,
            x: attrs.x,
            y: attrs.y,
            width: attrs.width as u32,
            height: attrs.height as u32,
            focused: false,
            marked: false,
            frame: None,
        }
    }

    pub fn id(&self) -> WindowID {
        self.id
    }

    pub fn attrs(&self) -> (i32, i32, u32, u32) {
        (self.x, self.y, self.width, self.height)
    }

    pub fn frame(&mut self) -> WindowID {
        if let Some(frame) = self.frame {
            frame
        } else {
            let frame = self.display.create_simple_window(
                self.display.root(),
                self.x,
                self.y,
                self.width as u32,
                self.height as u32,
                config::BORDER_WIDTH,
                config::BORDER_COLOR,
                config::BACKGROUND,
            );
            self.display.select_input(frame);
            self.display.add_to_save_set(self.id);
            self.display.reparent_window(self.id, frame, 0, 0);
            self.display.map_window(frame);

            self.frame = Some(frame);

            frame
        }
    }

    pub fn unframe(&mut self) {
        if let Some(frame) = self.frame {
            self.display.unmap_window(frame);
            self.display
                .reparent_window(self.id, self.display.root(), 0, 0);
            self.display.remove_from_save_set(self.id);
            self.display.destroy_window(frame);
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.display.move_window(self.frame(), x, y);
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        let w = w - 2 * config::BORDER_WIDTH;
        let h = h - 2 * config::BORDER_WIDTH;

        self.width = w;
        self.height = h;

        self.display.resize_window(self.frame(), w, h);
        self.display.resize_window(self.id, w, h);
    }

    pub fn focus(&mut self) {
        self.focused = true;
        self.display
            .set_window_border(self.frame(), config::FOCUSED_BORDER_COLOR);
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
        self.display
            .set_window_border(self.frame(), config::BORDER_COLOR);
    }

    pub fn mark(&mut self) {
        self.marked = true;
        self.display
            .set_window_border(self.frame(), config::MARKED_BORDER_COLOR);
    }

    pub fn unmark(&mut self) {
        self.marked = false;
        self.display
            .set_window_border(self.frame(), config::BORDER_COLOR);
    }

    pub fn map(&self) {
        self.display.map_window(self.id);
    }

    pub fn reparent(&self, parent: WindowID) {
        self.display.reparent_window(self.id, parent, 0, 0);
    }
}
