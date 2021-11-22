use std::sync::Arc;
use std::{thread, time};

use window::Window;

use crate::core::System;
use crate::window::{Backend, Event, EventHandler, Mode, WindowControlFlow};

mod core;
mod window;

fn main() {
    let mut system = System::new();

    let mut window = Window::new(&mut system, "Hello", 800, 600, Backend::Metal, Mode::Window);

    loop {
        if let WindowControlFlow::Exit = window.handle_events() {
            break;
        }
    }

    // thread::sleep(time::Duration::from_secs(10));
}
