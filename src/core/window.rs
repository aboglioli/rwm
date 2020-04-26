use x11::xlib;

pub type WindowID = u64;
pub type WindowAttributes = xlib::XWindowAttributes;
pub type WindowChanges = xlib::XWindowChanges;

#[derive(Debug)]
pub struct Window {
    pub id: WindowID,
    pub attrs: Option<WindowAttributes>,
    pub frame: Option<WindowID>,
}

impl Window {
    pub fn new(id: WindowID) -> Window {
        Window {
            id,
            attrs: None,
            frame: None,
        }
    }

    pub fn set_attributes(&mut self, attrs: WindowAttributes) {
        self.attrs = Some(attrs);
    }

    pub fn set_frame(&mut self, frame: WindowID) {
        self.frame = Some(frame);
    }
}
