use sdl::SDL_Window;

use super::pipeline::Pipeline;

// TODO Check if this struct will be useful
#[allow(dead_code)]
pub struct Renderer {
    pipeline: Pipeline,
}

impl Renderer {
    pub fn new(window: &mut SDL_Window) -> Self {
        let pipeline = Pipeline::from_sdl_window(window);

        Self { pipeline }
    }
}
