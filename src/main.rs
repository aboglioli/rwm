// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

use std::ffi::CString;
use std::mem;
use std::os::raw::*;
use std::ptr;

use x11::xlib;

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(not(feature = "xlib"))]
fn main() {
    panic!("this example requires `--features xlib`");
}

#[cfg(feature = "xlib")]
fn main() {
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("XOpenDisplay");
        }

        let key_str = CString::new("F2").unwrap();
        let key = xlib::XStringToKeysym(key_str.as_ptr()) as c_ulong;
        let mod_key = xlib::Mod1Mask;

        xlib::XGrabKey(
            display,
            xlib::XKeysymToKeycode(display, key) as c_int,
            mod_key,
            xlib::XDefaultRootWindow(display),
            1,
            xlib::GrabModeAsync,
            xlib::GrabModeAsync,
        );
        xlib::XGrabButton(
            display,
            1,
            mod_key,
            xlib::XDefaultRootWindow(display),
            xlib::True,
            (xlib::ButtonPressMask | xlib::ButtonReleaseMask | xlib::PointerMotionMask) as c_uint,
            xlib::GrabModeAsync,
            xlib::GrabModeAsync,
            0,
            0,
        );
        xlib::XGrabButton(
            display,
            3,
            mod_key,
            xlib::XDefaultRootWindow(display),
            xlib::True,
            (xlib::ButtonPressMask | xlib::ButtonReleaseMask | xlib::PointerMotionMask) as c_uint,
            xlib::GrabModeAsync,
            xlib::GrabModeAsync,
            0,
            0,
        );

        let mut event: xlib::XEvent = mem::uninitialized();
        let mut start: xlib::XButtonEvent = mem::uninitialized();
        start.subwindow = 0;
        let mut attr: xlib::XWindowAttributes = mem::uninitialized();

        loop {
            xlib::XNextEvent(display, &mut event);
            let event_type = event.get_type();

            match event.get_type() {
                xlib::KeyPress => {
                    if event.key.subwindow != 0 {
                        xlib::XRaiseWindow(display, event.key.subwindow);
                    }
                }
                xlib::ButtonPress => {
                    if event.button.subwindow != 0 {
                        xlib::XGetWindowAttributes(display, event.button.subwindow, &mut attr);
                        start = event.button;
                    }
                }
                xlib::MotionNotify => {
                    if start.subwindow != 0 {
                        let xdiff = event.button.x_root - start.x_root;
                        let ydiff = event.button.y_root - start.y_root;
                        xlib::XMoveResizeWindow(
                            display,
                            start.subwindow,
                            attr.x + if start.button == 1 { xdiff } else { 0 },
                            attr.y + if start.button == 1 { ydiff } else { 0 },
                            max(1, attr.width + if start.button == 3 { xdiff } else { 0 })
                                as c_uint,
                            max(1, attr.height + if start.button == 3 { ydiff } else { 0 })
                                as c_uint,
                        );
                    }
                }
                xlib::ButtonRelease => {
                    start.window = 0;
                }
                _ => (),
            }
        }
    }
}
