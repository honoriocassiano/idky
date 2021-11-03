use std::ffi::CStr;

pub struct System {}

impl System {
    pub fn new() -> Self {
        let status = unsafe { sdl::SDL_Init(sdl::SDL_INIT_EVERYTHING) };

        if status < 0 {
            panic!("Error initializing SDL: {}", Self::error_priv().unwrap());
        }

        System {}
    }

    fn error_priv() -> Option<String> {
        // TODO Improve this
        unsafe {
            CStr::from_ptr(sdl::SDL_GetError())
                .to_str()
                .ok()
                .map(|str| str.to_owned())
        }
    }

    pub fn get_error(&self) -> Option<String> {
        unsafe {
            CStr::from_ptr(sdl::SDL_GetError())
                .to_str()
                .ok()
                .map(|str| str.to_owned())
        }
    }
}
