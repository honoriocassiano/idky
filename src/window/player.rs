#![allow(non_upper_case_globals)]

use std::convert::TryInto;
use std::ffi::CString;
use std::path::PathBuf;

use sdl::{
    SDL_FreeSurface, SDL_KeyCode_SDLK_DOWN, SDL_KeyCode_SDLK_UP, SDL_KeyboardEvent, SDL_LoadBMP_RW,
    SDL_RWFromFile, SDL_Surface,
};

use crate::sdl::SdlEventType;
use crate::window::Event;

pub struct Player {
    surface: *mut SDL_Surface,
}

impl Player {
    pub fn new(bmp_path: PathBuf) -> Self {
        let path = CString::new(bmp_path.to_str().unwrap()).unwrap();
        let mode = CString::new("rb").unwrap();

        let surface: *mut SDL_Surface;

        unsafe { surface = SDL_LoadBMP_RW(SDL_RWFromFile(path.as_ptr(), mode.as_ptr()), 1) }

        if surface.is_null() {
            panic!("Failed to load image");
        }

        Self { surface }
    }

    pub fn handle_event(&mut self, event: Event) {
        if let SdlEventType::KeyDown = event.get_type() {
            let key_event: SDL_KeyboardEvent = event.try_into().unwrap();

            match key_event.keysym.sym {
                SDL_KeyCode_SDLK_DOWN => println!("DOWN!"),
                SDL_KeyCode_SDLK_UP => println!("UP!"),
                _ => {}
            }
        }
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        unsafe {
            SDL_FreeSurface(self.surface);
        }
    }
}
