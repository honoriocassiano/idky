#![allow(non_upper_case_globals)]

use std::ffi::CString;
use std::path::PathBuf;

use sdl::{SDL_FreeSurface, SDL_LoadBMP_RW, SDL_RWFromFile, SDL_Rect, SDL_Surface};

use crate::core::Vec2;
use crate::window::{RenderData, RenderTarget, Renderable};

pub struct Player {
    render_target: RenderTarget,
    surface: *mut SDL_Surface,
    position: Vec2,
}

impl Player {
    pub fn new(bmp_path: PathBuf, position: Vec2, render_target: RenderTarget) -> Self {
        let path = CString::new(bmp_path.to_str().unwrap()).unwrap();
        let mode = CString::new("rb").unwrap();

        let surface: *mut SDL_Surface;

        unsafe { surface = SDL_LoadBMP_RW(SDL_RWFromFile(path.as_ptr(), mode.as_ptr()), 1) }

        if surface.is_null() {
            panic!("Failed to load image");
        }

        Self {
            render_target,
            surface,
            position,
        }
    }

    pub fn move_by(&mut self, delta: Vec2) {
        self.position += delta;
    }

    fn get_sdl_rect(&self) -> SDL_Rect {
        let width = unsafe { (*self.surface).w as i32 };
        let height = unsafe { (*self.surface).h as i32 };

        let int_x = self.position.0.round() as i32 - (width >> 1);
        let int_y = self.position.1.round() as i32 - (height >> 1);

        SDL_Rect {
            x: int_x,
            y: int_y,
            w: width,
            h: height,
        }
    }
}

impl Renderable for Player {
    fn get_render_target(&self) -> RenderTarget {
        RenderTarget {
            surface: self.render_target.surface,
        }
    }

    fn get_render_data(&self) -> RenderData {
        RenderData {
            surface: self.surface,
            rect: None,
            dest_rect: self.get_sdl_rect(),
        }
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        unsafe {
            SDL_FreeSurface(self.surface);
        }
    }
}
