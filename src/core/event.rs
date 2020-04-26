use x11::xlib;

pub type CreateWindowEvent = xlib::XCreateWindowEvent;
pub type ConfigureRequestEvent = xlib::XConfigureRequestEvent;
pub type MapRequestEvent = xlib::XMapRequestEvent;
pub type DestroyWindowEvent = xlib::XDestroyWindowEvent;
pub type ReparentEvent = xlib::XReparentEvent;
pub type KeyEvent = xlib::XKeyEvent;
pub type ButtonEvent = xlib::XButtonEvent;
pub type MotionEvent = xlib::XMotionEvent;

pub enum Event {
    Create(CreateWindowEvent),
    ConfigureRequest(ConfigureRequestEvent),
    MapRequest(MapRequestEvent),
    Destroy(DestroyWindowEvent),
    Reparent(ReparentEvent),

    KeyPress(KeyEvent),
    KeyRelease(KeyEvent),

    ButtonPress(ButtonEvent),
    ButtonRelease(ButtonEvent),
    Motion(ButtonEvent, MotionEvent),
    Unknown,
}

impl From<xlib::XEvent> for Event {
    fn from(event: xlib::XEvent) -> Self {
        unsafe {
            match event.get_type() {
                xlib::CreateNotify => Self::Create(event.create_window),
                xlib::ConfigureRequest => Self::ConfigureRequest(event.configure_request),
                xlib::MapRequest => Self::MapRequest(event.map_request),
                xlib::DestroyNotify => Self::Destroy(event.destroy_window),
                xlib::ReparentNotify => Self::Reparent(event.reparent),

                xlib::KeyPress => Self::KeyPress(event.key),
                xlib::KeyRelease => Self::KeyRelease(event.key),

                xlib::ButtonPress => Self::ButtonPress(event.button),
                xlib::ButtonRelease => Self::ButtonRelease(event.button),
                xlib::MotionNotify => Self::Motion(event.button, event.motion),
                _ => Self::Unknown,
            }
        }
    }
}

pub const MODKEY: u32 = xlib::Mod1Mask;

pub const BUTTON_PRESS_MASK: u32 = xlib::ButtonPressMask as u32;
pub const BUTTON_RELEASE_MASK: u32 = xlib::ButtonReleaseMask as u32;
pub const POINTER_MOTION_MASK: u32 = xlib::PointerMotionMask as u32;
pub const GRAB_MODE_ASYNC: i32 = xlib::GrabModeAsync;
pub const GRAB_MODE_SYNC: i32 = xlib::GrabModeSync;
