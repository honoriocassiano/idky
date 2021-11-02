use std::ffi::{CStr, CString};
use std::ops::Deref;

use sdl::{SDL_CreateWindow, SDL_DestroyWindow, SDL_Init, SDL_INIT_EVERYTHING, SDL_INIT_VIDEO, SDL_Window};

pub struct Window {
    window: *mut SDL_Window,
}

fn get_error() -> Option<String> {
    unsafe {
        CStr::from_ptr(sdl::SDL_GetError()).to_str().ok()
            .map(|str| str.to_owned())
    }
}

impl Window {
    pub fn new(title: &str, width: i32, height: i32) -> Window {

        // TODO Split SDL initialization to another file
        let status = unsafe { SDL_Init(SDL_INIT_EVERYTHING) };

        if status < 0 {
            panic!("Error initializing SDL: {}", get_error().unwrap());
        }

        let window_name = CString::new(title).unwrap();

        let window = unsafe {
            SDL_CreateWindow(window_name.as_ptr(),
                             0,
                             0,
                             width,
                             height,
                             0) // TODO Add flags
        };

        if window.is_null() {
            panic!("Error initializing window: {}", get_error().unwrap());
        }

        Window {
            window
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyWindow(self.window);
        }
    }
}
