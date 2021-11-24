use window::Window;

use crate::core::System;
use crate::window::{Backend, Mode, WindowControlFlow};

mod core;
mod sdl;
mod window;

fn main() {
    let mut system = System::new();

    let mut window = Window::new(
        &mut system,
        "Hello",
        800,
        600,
        Backend::Vulkan,
        Mode::Window,
    );

    loop {
        if let WindowControlFlow::Exit = window.handle_events() {
            break;
        }
    }

    // thread::sleep(time::Duration::from_secs(10));
}
