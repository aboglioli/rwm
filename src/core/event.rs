use x11::xlib;

pub type CreateWindowEvent = xlib::XCreateWindowEvent;
pub type ConfigureEvent = xlib::XConfigureEvent;
pub type ReparentEvent = xlib::XReparentEvent;
pub type MapEvent = xlib::XMapEvent;
pub type UnmapEvent = xlib::XUnmapEvent;
pub type DestroyWindowEvent = xlib::XDestroyWindowEvent;

pub type ConfigureRequestEvent = xlib::XConfigureRequestEvent;
pub type MapRequestEvent = xlib::XMapRequestEvent;

pub type KeyEvent = xlib::XKeyEvent;
pub type ButtonEvent = xlib::XButtonEvent;
pub type MotionEvent = xlib::XMotionEvent;

pub enum Event {
    // Notify
    CreateNotify(CreateWindowEvent),
    ConfigureNotify(ConfigureEvent),
    ReparentNotify(ReparentEvent),
    MapNotify(MapEvent),
    UnmapNotify(UnmapEvent),
    DestroyNotify(DestroyWindowEvent),

    // Request
    ConfigureRequest(ConfigureRequestEvent),
    MapRequest(MapRequestEvent),

    // Keys
    KeyPress(KeyEvent),
    KeyRelease(KeyEvent),
    ButtonPress(ButtonEvent),
    ButtonRelease(ButtonEvent),
    MotionNotify(ButtonEvent, MotionEvent),
    Unknown,
}

impl From<xlib::XEvent> for Event {
    fn from(event: xlib::XEvent) -> Self {
        unsafe {
            match event.get_type() {
                xlib::CreateNotify => Self::CreateNotify(event.create_window),
                xlib::ConfigureNotify => Self::ConfigureNotify(event.configure),
                xlib::ReparentNotify => Self::ReparentNotify(event.reparent),
                xlib::MapNotify => Self::MapNotify(event.map),
                xlib::UnmapNotify => Self::UnmapNotify(event.unmap),
                xlib::DestroyNotify => Self::DestroyNotify(event.destroy_window),

                xlib::ConfigureRequest => Self::ConfigureRequest(event.configure_request),
                xlib::MapRequest => Self::MapRequest(event.map_request),

                xlib::KeyPress => Self::KeyPress(event.key),
                xlib::KeyRelease => Self::KeyRelease(event.key),
                xlib::ButtonPress => Self::ButtonPress(event.button),
                xlib::ButtonRelease => Self::ButtonRelease(event.button),
                xlib::MotionNotify => Self::MotionNotify(event.button, event.motion),
                _ => Self::Unknown,
            }
        }
    }
}

// pub const BUTTON_PRESS_MASK: u32 = xlib::ButtonPressMask as u32;
// pub const BUTTON_RELEASE_MASK: u32 = xlib::ButtonReleaseMask as u32;
// pub const POINTER_MOTION_MASK: u32 = xlib::PointerMotionMask as u32;
// pub const GRAB_MODE_ASYNC: i32 = xlib::GrabModeAsync;
// pub const GRAB_MODE_SYNC: i32 = xlib::GrabModeSync;
