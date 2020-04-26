use x11::xlib;

pub type WindowID = u64;
pub type WindowAttributes = xlib::XWindowAttributes;
pub type WindowChanges = xlib::XWindowChanges;

#[derive(Debug)]
pub struct Window {
    pub id: WindowID,
    pub attrs: WindowAttributes,
}

impl Window {
    pub fn new(id: WindowID, attrs: WindowAttributes) -> Window {
        Window { id, attrs }
    }
}
