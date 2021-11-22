use std::convert::TryInto;

use sdl::{SDL_Event, SDL_EventType_SDL_FIRSTEVENT};

use crate::sdl::SdlEventType;

pub struct Event {
    event: sdl::SDL_Event,
}

impl Event {
    pub fn get_raw_pointer_mut(&mut self) -> *mut SDL_Event {
        &mut self.event
    }

    pub fn get_type(&self) -> SdlEventType {
        unsafe { self.event.type_.try_into().unwrap() }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            event: SDL_Event {
                type_: SDL_EventType_SDL_FIRSTEVENT,
            },
        }
    }
}

pub enum WindowControlFlow {
    Continue,
    Exit,
}

pub struct EventHandler;

impl EventHandler {
    pub fn handle(&mut self, _event: &Event) -> WindowControlFlow {
        WindowControlFlow::Exit
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {}
    }
}
