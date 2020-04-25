use x11::xlib;

// TODO: C callbacks are not a good thing. They'll never be.
static mut ERROR_CODE: u8 = 0;

pub extern "C" fn error_handler(_: *mut xlib::Display, err: *mut xlib::XErrorEvent) -> i32 {
    if !err.is_null() {
        let err_code = unsafe { (*err).error_code };
        unsafe {
            ERROR_CODE = err_code;
        }
    }
    0
}

pub fn was_bad_access_err() -> bool {
    unsafe { ERROR_CODE == xlib::BadAccess }
}
