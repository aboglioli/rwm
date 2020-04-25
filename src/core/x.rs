use std::{mem, ptr};
use x11::xlib;

use crate::core::{error, event};

pub type Bool = i32;
pub type Window = u64;
pub type Cursor = u64;

pub struct Display {
    display_ptr: *mut xlib::Display,
    pub root: Window,
}

impl Display {
    pub fn open() -> Result<Display, String> {
        unsafe {
            let display_ptr = xlib::XOpenDisplay(ptr::null());
            if display_ptr.is_null() {
                return Err("Cannot open Display".to_string());
            }

            let root = xlib::XDefaultRootWindow(display_ptr);

            return Ok(Display { display_ptr, root });
        }
    }

    pub fn hook(&self) -> Result<(), String> {
        unsafe {
            xlib::XSetErrorHandler(Some(error::error_handler));
            xlib::XSelectInput(
                self.display_ptr,
                self.root,
                xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask,
            );
            xlib::XSync(self.display_ptr, 0);

            if error::was_bad_access_err() {
                return Err("Another WM is running".to_string());
            }
        }
        Ok(())
    }

    pub fn grab_button(
        &self,
        button: u32,
        modifiers: u32,
        grab_window: Window,
        owner_events: Bool,
        event_mask: u32,
        pointer_mode: i32,
        keyboard_mode: i32,
        confine_to: Window,
        cursor: Cursor,
    ) {
        unsafe {
            xlib::XGrabButton(
                self.display_ptr,
                button,
                modifiers,
                grab_window,
                owner_events,
                event_mask,
                pointer_mode,
                keyboard_mode,
                confine_to,
                cursor,
            );
        }
    }

    pub fn next_event(&self) -> event::Event {
        unsafe {
            let mut event: xlib::XEvent = mem::MaybeUninit::uninit().assume_init();
            xlib::XNextEvent(self.display_ptr, &mut event);

            match event.get_type() {
                xlib::CreateNotify => event::Event::Create(event.create_window),
                xlib::DestroyNotify => event::Event::Destroy(event.destroy_window),
                xlib::ReparentNotify => event::Event::Reparent(event.reparent),

                xlib::KeyPress => event::Event::KeyPress(event.key),
                xlib::KeyRelease => event::Event::KeyRelease(event.key),

                xlib::ButtonPress => event::Event::ButtonPress(event.button),
                xlib::ButtonRelease => event::Event::ButtonRelease(event.button),
                xlib::MotionNotify => event::Event::Motion(event.button, event.motion),
                _ => event::Event::Unknown,
            }
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display_ptr);
        }
    }
}
