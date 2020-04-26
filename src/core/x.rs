use std::{mem, ptr};
use x11::xlib;

use crate::core::{cursor, error, event, window};

pub type Bool = i32;

pub struct Display {
    ptr: *mut xlib::Display,
    root: window::WindowID,
}

impl Display {
    pub fn open() -> Result<Display, String> {
        unsafe {
            // Open connection
            let ptr = xlib::XOpenDisplay(ptr::null());
            if ptr.is_null() {
                return Err("Cannot open Display".to_string());
            }

            // Get root window
            let root = xlib::XDefaultRootWindow(ptr);

            // Check other WMs
            xlib::XSetErrorHandler(Some(error::error_handler));
            xlib::XSelectInput(
                ptr,
                root,
                xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask,
            );
            xlib::XSync(ptr, 0);

            if error::last_error() == xlib::BadAccess {
                return Err("Another WM is running".to_string());
            }

            return Ok(Display { ptr, root });
        }
    }

    pub fn root(&self) -> window::WindowID {
        self.root
    }

    pub fn get_windows(&self) -> Result<Vec<window::Window>, String> {
        let mut windows = Vec::new();

        unsafe {
            let mut d1 = mem::MaybeUninit::uninit().assume_init();
            let mut d2 = mem::MaybeUninit::uninit().assume_init();
            let mut wins = mem::MaybeUninit::uninit().assume_init();
            let mut num = 0;

            if xlib::XQueryTree(self.ptr, self.root, &mut d1, &mut d2, &mut wins, &mut num) == 0 {
                return Err("XQueryTree failed".to_string());
            }

            for i in 0..num {
                let i = i as isize;
                let w = *wins.offset(i as isize);
                let mut attrs = mem::MaybeUninit::uninit().assume_init();

                if xlib::XGetWindowAttributes(self.ptr, w, &mut attrs) == 0 {
                    return Err("XGetWindowAttributes failed".to_string());
                }

                let mut window = window::Window::new(w);
                window.attrs = Some(attrs);
                windows.push(window);
            }
        }

        Ok(windows)
    }

    pub fn configure_window(
        &self,
        w: window::WindowID,
        value_mask: u32,
        changes: &mut window::WindowChanges,
    ) {
        unsafe {
            xlib::XConfigureWindow(self.ptr, w, value_mask, changes);
        }
    }

    pub fn move_window(&self, w: window::WindowID, x: i32, y: i32) {
        unsafe {
            xlib::XMoveWindow(self.ptr, w, x, y);
        }
    }

    pub fn resize_window(&self, w: window::WindowID, width: u32, height: u32) {
        unsafe {
            xlib::XResizeWindow(self.ptr, w, width, height);
        }
    }

    pub fn grab_button(
        &self,
        button: u32,
        modifiers: u32,
        grab_window: window::WindowID,
        owner_events: Bool,
        event_mask: u32,
        pointer_mode: i32,
        keyboard_mode: i32,
        confine_to: window::WindowID,
        cursor: cursor::CursorID,
    ) {
        unsafe {
            xlib::XGrabButton(
                self.ptr,
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
            xlib::XNextEvent(self.ptr, &mut event);
            event::Event::from(event)
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.ptr);
        }
    }
}
