use std::convert::TryInto;
use std::fmt::{Display, Formatter};

use sdl::{SDL_Event, SDL_EventType_SDL_FIRSTEVENT, SDL_KeyboardEvent};

use crate::sdl::SdlEventType;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone, Debug)]
pub struct EventConversionError(SdlEventType);

impl Display for EventConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Invalid type, current type is: {:?}", self.0).as_str())
    }
}

impl TryInto<SDL_KeyboardEvent> for Event {
    type Error = EventConversionError;

    fn try_into(self) -> Result<SDL_KeyboardEvent, Self::Error> {
        return match self.get_type() {
            SdlEventType::KeyDown | SdlEventType::KeyUp => unsafe { Ok(self.event.key) },
            t => Err(EventConversionError(t)),
        };
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
    #[allow(unused)]
    pub fn handle(&mut self, _event: &Event) -> WindowControlFlow {
        WindowControlFlow::Exit
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {}
    }
}
