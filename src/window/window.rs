use std::ffi::CString;
use std::os::raw::c_int;
use std::path::Path;
use std::time::Instant;

use sdl::{
    SDL_Scancode_SDL_SCANCODE_DOWN, SDL_Scancode_SDL_SCANCODE_UP, SDL_Surface, SDL_Window,
    SDL_WindowFlags_SDL_WINDOW_BORDERLESS, SDL_WindowFlags_SDL_WINDOW_FULLSCREEN,
    SDL_WindowFlags_SDL_WINDOW_METAL, SDL_WindowFlags_SDL_WINDOW_OPENGL,
    SDL_WindowFlags_SDL_WINDOW_VULKAN, SDL_WINDOWPOS_CENTERED_MASK,
};

use crate::core::{System, Vec2, Vector};
use crate::sdl::SdlEventType;
use crate::window::{
    event::EventHandler, player::Player, Event, RenderTarget, Renderable, Renderer,
    WindowControlFlow,
};

pub struct Window<'a> {
    system: &'a System,
    window: *mut SDL_Window,
    surface: *mut SDL_Surface,
    event_handler: EventHandler,
    player: Player,
    start_time: Instant,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Backend {
    Software = 0,
    Metal = SDL_WindowFlags_SDL_WINDOW_METAL,
    Opengl = SDL_WindowFlags_SDL_WINDOW_OPENGL,
    Vulkan = SDL_WindowFlags_SDL_WINDOW_VULKAN,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Mode {
    Window = 0,
    Fullscreen = SDL_WindowFlags_SDL_WINDOW_FULLSCREEN,
    Borderless = SDL_WindowFlags_SDL_WINDOW_BORDERLESS,
}

impl<'a> Window<'a> {
    pub fn new(
        system: &'a mut System,
        title: &str,
        width: i32,
        height: i32,
        backend: Backend,
        mode: Mode,
    ) -> Self {
        let window_name = CString::new(title).unwrap();
        let flags = backend as u32 | mode as u32;

        let window = unsafe {
            sdl::SDL_CreateWindow(
                window_name.as_ptr(),
                SDL_WINDOWPOS_CENTERED_MASK as c_int,
                SDL_WINDOWPOS_CENTERED_MASK as c_int,
                width,
                height,
                flags,
            )
        };

        if window.is_null() {
            panic!("Error initializing window: {}", system.get_error().unwrap());
        }

        let renderer = Renderer::new(unsafe { window.as_mut() }.unwrap());

        let surface = unsafe { sdl::SDL_GetWindowSurface(window) };

        let path = Path::new("./assets/example.bmp");
        let player = Player::new(path.to_path_buf(), Vec2::zero(), RenderTarget { surface });

        Self {
            system,
            window,
            surface,
            event_handler: EventHandler::default(),
            player,
            start_time: Instant::now(),
        }
    }

    #[allow(unused)]
    pub(super) unsafe fn get_raw_window(&self) -> &SDL_Window {
        self.window.as_ref().unwrap()
    }

    pub fn run(&mut self) {
        self.start_time = Instant::now();

        loop {
            if let WindowControlFlow::Exit = self.handle_events() {
                break;
            }

            self.render();
        }
    }

    pub fn render(&mut self) {
        unsafe {
            sdl::SDL_FillRect(self.surface, std::ptr::null(), 0x00000000);
        }

        self.player.render();

        unsafe { sdl::SDL_UpdateWindowSurface(self.window) };
    }

    pub fn handle_events(&mut self) -> WindowControlFlow {
        let mut event: Event = Default::default();

        while unsafe { sdl::SDL_PollEvent(event.get_raw_pointer_mut()) } != 0 {
            match event.get_type() {
                SdlEventType::Quit => return WindowControlFlow::Exit,
                _ => {}
            };
        }

        let mut size = 0i32;
        let state: &[u8];

        unsafe {
            let temp = sdl::SDL_GetKeyboardState(&mut size as *mut i32);
            state = std::slice::from_raw_parts(temp, size as usize);
        }

        let index_up = SDL_Scancode_SDL_SCANCODE_UP as usize;
        let index_down = SDL_Scancode_SDL_SCANCODE_DOWN as usize;

        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.start_time).as_secs_f32();

        let base_speed = 300.0;

        if state[index_up] != 0 {
            self.player.move_by(Vec2(0.0, -base_speed) * delta_time);
        }

        if state[index_down] != 0 {
            self.player.move_by(Vec2(0.0, base_speed) * delta_time);
        }

        self.start_time = current_time;

        WindowControlFlow::Continue
    }
}

impl<'a> Drop for Window<'a> {
    fn drop(&mut self) {
        unsafe {
            sdl::SDL_DestroyWindow(self.window);
        }
    }
}
