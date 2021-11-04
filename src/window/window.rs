use std::ffi::CString;
use std::sync::Arc;

use sdl::{
    SDL_CreateWindow, SDL_DestroyWindow, SDL_Window, SDL_WindowFlags_SDL_WINDOW_BORDERLESS,
    SDL_WindowFlags_SDL_WINDOW_FULLSCREEN, SDL_WindowFlags_SDL_WINDOW_METAL,
    SDL_WindowFlags_SDL_WINDOW_OPENGL, SDL_WindowFlags_SDL_WINDOW_VULKAN,
};

use crate::core::System;
use crate::window::event::EventHandler;

pub struct Window<'a> {
    system: &'a System,
    window: *mut SDL_Window,
    event_handlers: Vec<Arc<dyn EventHandler>>,
}

pub struct EventHandlerId<'a> {
    window: &'a Window<'a>,
    index: usize,
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
            event_handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, event_handler: Arc<dyn EventHandler>) -> EventHandlerId {
        self.event_handlers.push(event_handler);

        EventHandlerId {
            window: self,
            // FIXME This will cause a bug after remove some handler from Vec
            // Use a slab or a hashtable instead
            index: self.event_handlers.len(),
        }
    }
}

impl<'a> Drop for Window<'a> {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyWindow(self.window);
        }
    }
}
