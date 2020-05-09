use std::rc::Rc;
use x11::xlib;

use crate::core::{config, node, x};

pub type WindowID = u64;
pub type WindowAttributes = xlib::XWindowAttributes;
pub type WindowChanges = xlib::XWindowChanges;

pub struct Position {
    x: i32,
    y: i32,
}

pub struct Size {
    width: u32,
    height: u32,
}

pub struct Window {
    // Open display
    display: Rc<x::Display>,

    id: WindowID,
    position: Position,
    size: Size,

    focused: bool,
    marked: bool,

    frame: WindowID,
}

#[allow(dead_code)]
impl Window {
    pub fn new(display: &Rc<x::Display>, id: WindowID, attrs: WindowAttributes) -> Window {
        // Create frame
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
        display.reparent_window(id, frame, 0, 0);
        display.map_window(frame);

        Window {
            display: Rc::clone(display),
            id,
            position: Position {
                x: attrs.y,
                y: attrs.y,
            },
            size: Size {
                width: attrs.width as u32,
                height: attrs.height as u32,
            },
            focused: false,
            marked: false,
            frame,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn frame(&self) -> WindowID {
        self.frame
    }

    pub fn unframe(&self) {
        self.display.unmap_window(self.frame);
        self.display
            .reparent_window(self.id, self.display.root(), 0, 0);
        self.display.remove_from_save_set(self.id);
        self.display.destroy_window(self.frame);
    }
}

impl node::Node for Window {
    fn id(&self) -> node::NodeID {
        self.id
    }

    fn is(&self, id: node::NodeID) -> bool {
        self.id() == id
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.position = Position { x, y };
        self.display.move_window(self.frame, x, y);
    }

    fn set_size(&mut self, width: u32, height: u32) {
        let width = width - 2 * config::BORDER_WIDTH;
        let height = height - 2 * config::BORDER_WIDTH;

        self.size = Size { width, height };

        self.display.resize_window(self.frame, width, height);
        self.display.resize_window(self.id, width, height);
    }

    fn focus(&mut self) {
        self.focused = true;
        self.display
            .set_window_border(self.frame, config::FOCUSED_BORDER_COLOR);
    }

    fn unfocus(&mut self) {
        self.focused = false;
        self.display
            .set_window_border(self.frame, config::BORDER_COLOR);
    }

    fn mark(&mut self) {
        self.marked = true;
        self.display
            .set_window_border(self.frame, config::MARKED_BORDER_COLOR);
    }

    fn unmark(&mut self) {
        self.marked = false;
        self.display
            .set_window_border(self.frame, config::BORDER_COLOR);
    }

    fn map(&self) {
        self.display.map_window(self.id);
    }

    fn reparent(&self, parent: WindowID) {
        self.display.reparent_window(self.id, parent, 0, 0);
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.unframe();
    }
}
