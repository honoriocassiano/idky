use std::{thread, time};

use window::Window;

use crate::window::{Backend, Mode};

mod window;

fn main() {
    let _window = Window::new("Hello", 800, 600, Backend::Metal, Mode::Window);

    thread::sleep(time::Duration::from_secs(10));
}
