use std::ffi::{CStr, CString};

use sdl::{
    SDL_CreateWindow, SDL_DestroyWindow, SDL_Init, SDL_INIT_EVERYTHING, SDL_INIT_VIDEO,
    SDL_Window, SDL_WindowFlags,
    SDL_WindowFlags_SDL_WINDOW_BORDERLESS, SDL_WindowFlags_SDL_WINDOW_FULLSCREEN,
    SDL_WindowFlags_SDL_WINDOW_METAL, SDL_WindowFlags_SDL_WINDOW_OPENGL, SDL_WindowFlags_SDL_WINDOW_VULKAN,
};

pub struct Window {
    window: *mut SDL_Window,
}

fn get_error() -> Option<String> {
    unsafe {
        CStr::from_ptr(sdl::SDL_GetError())
            .to_str()
            .ok()
            .map(|str| str.to_owned())
    }
}

#[allow(dead_code)]
#[repr(u32)]
pub enum Backend {
    Software = 0,
    Metal = SDL_WindowFlags_SDL_WINDOW_METAL,
    Opengl = SDL_WindowFlags_SDL_WINDOW_OPENGL,
    Vulkan = SDL_WindowFlags_SDL_WINDOW_VULKAN,
}

#[allow(dead_code)]
#[repr(u32)]
pub enum Mode {
    Window = 0,
    Fullscreen = SDL_WindowFlags_SDL_WINDOW_FULLSCREEN,
    Borderless = SDL_WindowFlags_SDL_WINDOW_BORDERLESS,
}

impl Window {
    pub fn new(title: &str, width: i32, height: i32, backend: Backend, mode: Mode) -> Window {
        // TODO Split SDL initialization to another file
        let status = unsafe { SDL_Init(SDL_INIT_EVERYTHING) };

        if status < 0 {
            panic!("Error initializing SDL: {}", get_error().unwrap());
        }

        let window_name = CString::new(title).unwrap();
        let flags = backend as u32 | mode as u32;

        let window = unsafe { SDL_CreateWindow(window_name.as_ptr(), 0, 0, width, height, flags) };

        if window.is_null() {
            panic!("Error initializing window: {}", get_error().unwrap());
        }

        Window { window }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyWindow(self.window);
        }
    }
}
