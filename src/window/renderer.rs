use sdl::SDL_Window;

use super::pipeline::Pipeline;

pub struct Renderer {
    pipeline: Pipeline,
}

impl Renderer {
    pub fn new(window: &mut SDL_Window) -> Self {
        let pipeline = Pipeline::from_sdl_window(window);

        Self { pipeline }
    }
}
