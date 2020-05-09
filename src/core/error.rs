use std::error;
use std::fmt;
use x11::xlib;

/**
 * C callbacks are not a good thing to use in Rust. They'll never be.
 * TODO: make a trampoline function
 */
static mut LAST_ERROR_CODE: u8 = 0;

pub extern "C" fn error_handler(_: *mut xlib::Display, err: *mut xlib::XErrorEvent) -> i32 {
    if !err.is_null() {
        let err_code = unsafe { (*err).error_code };
        unsafe {
            LAST_ERROR_CODE = err_code;
        }
    }
    0
}

pub fn last_error() -> u8 {
    unsafe { LAST_ERROR_CODE }
}

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    fn new(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
