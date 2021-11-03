use std::{thread, time};

use window::Window;

use crate::core::System;
use crate::window::{Backend, Mode};

mod core;
mod window;

fn main() {
    let mut system = System::new();

    let _window = Window::new(&mut system, "Hello", 800, 600, Backend::Metal, Mode::Window);

    thread::sleep(time::Duration::from_secs(10));
}
