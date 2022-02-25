use sdl::SDL_Window;

use super::pipeline::Pipeline;

// TODO Check if this struct will be useful
#[allow(dead_code)]
pub struct Renderer<'a> {
    pipeline: Pipeline<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(window: &'a mut SDL_Window) -> Self {
        let pipeline = Pipeline::from_sdl_window(window);

        Self { pipeline }
    }

    pub fn draw(&mut self) {
        self.pipeline.draw();
    }
}
