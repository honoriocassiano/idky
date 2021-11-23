use std::ffi::CString;
use std::path::PathBuf;

use sdl::{SDL_FreeSurface, SDL_LoadBMP_RW, SDL_RWFromFile, SDL_Surface};

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
}

impl Drop for Player {
    fn drop(&mut self) {
        unsafe {
            SDL_FreeSurface(self.surface);
        }
    }
}
