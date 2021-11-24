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

    fn get_sdl_rect(&self) -> SDL_Rect {
        // FIXME Calculate using current position
        SDL_Rect {
            x: 0,
            y: 0,
            w: 80,
            h: 80,
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
