use core::ptr;
use std::convert::TryInto;
use std::ffi::CString;
use std::path::Path;

use sdl::{
    SDL_CreateWindow, SDL_DestroyWindow, SDL_FillRect, SDL_GetWindowSurface, SDL_KeyCode_SDLK_DOWN,
    SDL_KeyCode_SDLK_UP, SDL_KeyboardEvent, SDL_PollEvent, SDL_Surface, SDL_UpdateWindowSurface,
    SDL_Window, SDL_WindowFlags_SDL_WINDOW_BORDERLESS, SDL_WindowFlags_SDL_WINDOW_FULLSCREEN,
    SDL_WindowFlags_SDL_WINDOW_METAL, SDL_WindowFlags_SDL_WINDOW_OPENGL,
    SDL_WindowFlags_SDL_WINDOW_VULKAN,
};

use crate::core::{System, Vec2, Vector};
use crate::sdl::SdlEventType;
use crate::window::event::EventHandler;
use crate::window::player::Player;
use crate::window::{Event, RenderTarget, Renderable, WindowControlFlow};

pub struct Window<'a> {
    system: &'a System,
    window: *mut SDL_Window,
    surface: *mut SDL_Surface,
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

        let surface = unsafe { SDL_GetWindowSurface(window) };

        let path = Path::new("./assets/example.bmp");
        let player = Player::new(
            path.to_path_buf(),
            Vec2::zero(),
            RenderTarget {
                surface: window_surface,
            },
        );

        Self {
            system,
            window,
            surface,
            event_handler: EventHandler::default(),
            player,
        }
    }

    pub fn render(&mut self) {
        self.player.render();

        unsafe { SDL_UpdateWindowSurface(self.window) };
    }

    pub fn handle_events(&mut self) -> WindowControlFlow {
        let mut event: Event = Default::default();

        while unsafe { SDL_PollEvent(event.get_raw_pointer_mut()) } != 0 {
            match event.get_type() {
                SdlEventType::Quit => return WindowControlFlow::Exit,
                SdlEventType::KeyDown => {
                    let key_event: SDL_KeyboardEvent = event.try_into().unwrap();

                    match key_event.keysym.sym {
                        SDL_KeyCode_SDLK_DOWN => println!("DOWN!"),
                        SDL_KeyCode_SDLK_UP => println!("UP!"),
                        _ => {}
                    }
                }
                _ => {}
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
