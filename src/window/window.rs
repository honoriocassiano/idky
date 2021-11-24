use std::ffi::CString;
use std::path::{Path, PathBuf};

use sdl::{
    SDL_CreateWindow, SDL_DestroyWindow, SDL_PollEvent, SDL_Window,
    SDL_WindowFlags_SDL_WINDOW_BORDERLESS, SDL_WindowFlags_SDL_WINDOW_FULLSCREEN,
    SDL_WindowFlags_SDL_WINDOW_METAL, SDL_WindowFlags_SDL_WINDOW_OPENGL,
    SDL_WindowFlags_SDL_WINDOW_VULKAN,
};

use crate::core::System;
use crate::sdl::SdlEventType;
use crate::window::event::EventHandler;
use crate::window::player::Player;
use crate::window::{Event, WindowControlFlow};

pub struct Window<'a> {
    system: &'a System,
    window: *mut SDL_Window,
    event_handler: EventHandler,
    player: Player,
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

impl<'a> Window<'a> {
    pub fn new(
        system: &'a mut System,
        title: &str,
        width: i32,
        height: i32,
        backend: Backend,
        mode: Mode,
    ) -> Self {
        let window_name = CString::new(title).unwrap();
        let flags = backend as u32 | mode as u32;

        let window = unsafe { SDL_CreateWindow(window_name.as_ptr(), 0, 0, width, height, flags) };

        if window.is_null() {
            panic!("Error initializing window: {}", system.get_error().unwrap());
        }

        let path = Path::new("./assets/example.bmp");
        let player = Player::new(path.to_path_buf());

        Self {
            system,
            window,
            event_handler: EventHandler::default(),
            player,
        }
    }

    pub fn handle_events(&mut self) -> WindowControlFlow {
        let mut event: Event = Default::default();

        while unsafe { SDL_PollEvent(event.get_raw_pointer_mut()) } != 0 {
            match event.get_type() {
                SdlEventType::Quit => return WindowControlFlow::Exit,
                _ => self.player.handle_event(event),
            };
        }

        WindowControlFlow::Continue
    }
}

impl<'a> Drop for Window<'a> {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyWindow(self.window);
        }
    }
}
