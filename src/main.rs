use std::{thread, time};
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::thread::sleep;

use window::Window;

mod window;

fn main() {
    let _window = Window::new("Hello", 800, 600);

    sleep(time::Duration::from_secs(10));
}
