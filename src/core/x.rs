use std::{mem, ptr};
use x11::xlib;

use crate::core::{cursor, error, event, window};

pub type Bool = i32;
pub const IS_VIEWABLE: i32 = xlib::IsViewable;

pub struct Display {
    ptr: *mut xlib::Display,
    root: window::WindowID,
}

#[allow(dead_code)]
impl Display {
    // Open display
    pub fn open() -> Result<Display, String> {
        unsafe {
            // Open connection
            let ptr = xlib::XOpenDisplay(ptr::null());
            if ptr.is_null() {
                return Err("Cannot open Display".to_string());
            }

            // Get root window
            let root = xlib::XDefaultRootWindow(ptr);

            let display = Display { ptr, root };

            // Global error handler
            xlib::XSetErrorHandler(Some(error::error_handler));

            // Check other WMs
            display.select_input(display.root());
            display.sync();

            if error::last_error() == xlib::BadAccess {
                return Err("Another WM is running".to_string());
            }

            Ok(display)
        }
    }

    pub fn root(&self) -> window::WindowID {
        self.root
    }

    // Global
    pub fn create_simple_window(
        &self,
        parent: window::WindowID,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        border: u64,
        background: u64,
    ) -> window::WindowID {
        unsafe {
            xlib::XCreateSimpleWindow(
                self.ptr,
                parent,
                x,
                y,
                width,
                height,
                border_width,
                border,
                background,
            )
        }
    }

    pub fn destroy_window(&self, w: window::WindowID) {
        unsafe {
            xlib::XDestroyWindow(self.ptr, w);
        }
    }

    pub fn select_input(&self, w: window::WindowID) {
        unsafe {
            xlib::XSelectInput(
                self.ptr,
                w,
                xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask,
            );
        }
    }

    pub fn sync(&self) {
        unsafe {
            xlib::XSync(self.ptr, 0);
        }
    }

    pub fn query_tree(
        &self,
        w: window::WindowID,
    ) -> Result<(window::WindowID, window::WindowID, Vec<window::WindowID>), String> {
        unsafe {
            xlib::XGrabServer(self.ptr);

            let mut root_return = mem::MaybeUninit::uninit().assume_init();
            let mut parent_return = mem::MaybeUninit::uninit().assume_init();
            let mut w_ptr = mem::MaybeUninit::uninit().assume_init();
            let mut num = 0;

            if xlib::XQueryTree(
                self.ptr,
                w,
                &mut root_return,
                &mut parent_return,
                &mut w_ptr,
                &mut num,
            ) == 0
            {
                return Err("XQueryTree failed".to_string());
            }

            let mut win_ids = Vec::new();
            for i in 0..num {
                let i = i as isize;
                let w = *w_ptr.offset(i);
                win_ids.push(w);
            }
            xlib::XFree(w_ptr as *mut core::ffi::c_void);
            xlib::XUngrabServer(self.ptr);

            Ok((root_return, parent_return, win_ids))
        }
    }

    // Window configuration and move/resize
    pub fn configure_window(
        &self,
        w: window::WindowID,
        value_mask: u64,
        changes: &mut window::WindowChanges,
    ) {
        unsafe {
            xlib::XConfigureWindow(self.ptr, w, value_mask as u32, changes);
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

    pub fn add_to_save_set(&self, w: window::WindowID) {
        unsafe {
            xlib::XAddToSaveSet(self.ptr, w);
        }
    }

    pub fn remove_from_save_set(&self, w: window::WindowID) {
        unsafe {
            xlib::XRemoveFromSaveSet(self.ptr, w);
        }
    }

    pub fn map_window(&self, w: window::WindowID) {
        unsafe {
            xlib::XMapWindow(self.ptr, w);
        }
    }

    pub fn unmap_window(&self, w: window::WindowID) {
        unsafe {
            xlib::XUnmapWindow(self.ptr, w);
        }
    }

    pub fn reparent_window(&self, w: window::WindowID, parent: window::WindowID, x: i32, y: i32) {
        unsafe {
            xlib::XReparentWindow(self.ptr, w, parent, x, y);
        }
    }

    pub fn get_window_attributes(
        &self,
        w: window::WindowID,
    ) -> Result<window::WindowAttributes, String> {
        unsafe {
            let mut attrs = mem::MaybeUninit::uninit().assume_init();
            if xlib::XGetWindowAttributes(self.ptr, w, &mut attrs) == 0 {
                return Err("XGetWindowAttributes failed".to_string());
            }
            Ok(attrs)
        }
    }

    // Events
    pub fn grab_key(
        &self,
        keycode: i32,
        modifiers: u32,
        grab_window: window::WindowID,
        owner_events: Bool,
        pointer_mode: i32,
        keyboard_mode: i32,
    ) {
        unsafe {
            xlib::XGrabKey(
                self.ptr,
                keycode,
                modifiers,
                grab_window,
                owner_events,
                pointer_mode,
                keyboard_mode,
            );
        }
    }

    pub fn set_window_border(&self, w: window::WindowID, color: u64) {
        unsafe {
            xlib::XSetWindowBorder(self.ptr, w, color);
        }
    }

    pub fn ungrab_key(&self, keycode: i32, modifiers: u32, grab_window: window::WindowID) {
        unsafe {
            xlib::XUngrabKey(self.ptr, keycode, modifiers, grab_window);
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

    pub fn ungrab_button(&self, button: u32, modifiers: u32, grab_window: window::WindowID) {
        unsafe {
            xlib::XUngrabButton(self.ptr, button, modifiers, grab_window);
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
