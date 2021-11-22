pub struct Event<'a> {
    event: &'a sdl::SDL_Event,
}

impl<'a> Event<'a> {
    pub fn new(event: &'a sdl::SDL_Event) -> Self {
        Self { event }
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
