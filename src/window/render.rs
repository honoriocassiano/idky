use core::ptr;

use sdl::{SDL_Rect, SDL_Surface, SDL_UpperBlit};

#[derive(Copy, Clone)]
pub struct RenderData {
    pub surface: *mut SDL_Surface,
    pub rect: Option<SDL_Rect>,
    pub dest_rect: SDL_Rect,
}

#[derive(Copy, Clone)]
pub struct RenderTarget {
    pub surface: *mut SDL_Surface,
}

pub trait Renderable {
    fn get_render_target(&self) -> RenderTarget;

    fn get_render_data(&self) -> RenderData;

    fn render(&mut self) {
        let mut render_data = self.get_render_data();
        let render_target = self.get_render_target();

        let src_rect = render_data
            .rect
            .map(|r| &r as *const SDL_Rect)
            .unwrap_or(ptr::null());
        let dest_rect = &mut render_data.dest_rect as *mut SDL_Rect;

        unsafe {
            SDL_UpperBlit(
                render_data.surface,
                src_rect,
                render_target.surface,
                dest_rect,
            );
        }
    }
}
