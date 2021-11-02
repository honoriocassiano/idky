use std::{thread, time};
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::thread::sleep;

use sdl::*;

fn get_error() -> Option<String> {
    unsafe {
        CStr::from_ptr(SDL_GetError()).to_str().ok()
            .map(|str| str.to_owned())
    }
}

fn print_if_error(message: &'static str) {
    if let Some(err) = get_error() {
        println!("{}: {}", message, err);
    }
}

fn main() {
    let status = unsafe { SDL_Init(SDL_INIT_VIDEO) };

    if status < 0 {
        print_if_error("Error initializing SDL");

        return;
    }

    let window_name = CString::new("Hello World").unwrap();

    let window = unsafe {
        SDL_CreateWindow(window_name.as_ptr(),
                         0 as c_int,
                         0 as c_int,
                         800,
                         600,
                         0) // No flags
    };

    if window.is_null() {
        print_if_error("Error initializing window");

        return;
    }

    sleep(time::Duration::from_secs(10));

    unsafe { SDL_DestroyWindow(window) };
}
