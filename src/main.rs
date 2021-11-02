use std::{thread, time};
use std::ffi::CStr;

use sdl::*;

fn main() {
    unsafe {
        let status = SDL_Init(SDL_INIT_VIDEO);

        if status < 0 {
            let error = CStr::from_ptr(SDL_GetError()).to_str().unwrap();

            println!("Error initializing SDL: {}", error);
        } else {
            println!("Ok");
        }
    }
}
