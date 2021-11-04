use std::sync::Arc;
use std::{thread, time};

use window::Window;

use crate::core::System;
use crate::window::{Backend, Event, EventHandler, Mode, WindowControlFlow};

mod core;
mod window;

struct HandlerExample {}

impl EventHandler for HandlerExample {
    fn handle(&mut self, _event: &Event) -> WindowControlFlow {
        todo!()
    }
}

fn main() {
    let mut system = System::new();

    let mut window = Window::new(&mut system, "Hello", 800, 600, Backend::Metal, Mode::Window);

    window.add_handler(Arc::new(HandlerExample {}));

    thread::sleep(time::Duration::from_secs(10));
}
