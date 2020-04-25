use x11::xlib;

pub enum Event {
    Create(xlib::XCreateWindowEvent),
    Destroy(xlib::XDestroyWindowEvent),
    Reparent(xlib::XReparentEvent),

    KeyPress(xlib::XKeyEvent),
    KeyRelease(xlib::XKeyEvent),

    ButtonPress(xlib::XButtonEvent),
    ButtonRelease(xlib::XButtonEvent),
    Motion(xlib::XButtonEvent, xlib::XMotionEvent),
    Unknown,
}

pub const BUTTON_PRESS_MASK: u32 = xlib::ButtonPressMask as u32;
pub const BUTTON_RELEASE_MASK: u32 = xlib::ButtonReleaseMask as u32;
pub const POINTER_MOTION_MASK: u32 = xlib::PointerMotionMask as u32;
pub const GRAB_MODE_ASYNC: i32 = xlib::GrabModeAsync;
// pub const GRAB_MODE_SYNC: i32 = xlib::GrabModeSync;
