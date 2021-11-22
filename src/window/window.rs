use std::ffi::CString;
use std::sync::Arc;

use sdl::{
    SDL_CreateWindow, SDL_DestroyWindow, SDL_Event, SDL_EventType_SDL_FIRSTEVENT,
    SDL_EventType_SDL_QUIT, SDL_PollEvent, SDL_Window, SDL_WindowFlags_SDL_WINDOW_BORDERLESS,
    SDL_WindowFlags_SDL_WINDOW_FULLSCREEN, SDL_WindowFlags_SDL_WINDOW_METAL,
    SDL_WindowFlags_SDL_WINDOW_OPENGL, SDL_WindowFlags_SDL_WINDOW_VULKAN,
};

use crate::core::System;
use crate::window::event::EventHandler;
use crate::window::WindowControlFlow;

pub struct Window<'a> {
    system: &'a System,
    window: *mut SDL_Window,
    event_handler: EventHandler,
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

        Self {
            system,
            window,
            event_handler: EventHandler::default(),
        }
    }

    pub fn handle_events(&mut self) -> WindowControlFlow {
        let mut event = SDL_Event {
            type_: SDL_EventType_SDL_FIRSTEVENT,
        };

        let mut event_pointer: *mut SDL_Event = &mut event;

        while unsafe { SDL_PollEvent(event_pointer) } != 0 {
            match unsafe { event.type_ } {
                SDL_EventType_SDL_QUIT => {
                    return WindowControlFlow::Exit;
                }
                _ => continue,
            }
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
