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

pub trait EventHandler {
    fn handle(&mut self, event: &Event) -> WindowControlFlow;
}
