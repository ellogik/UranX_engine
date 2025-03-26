#![allow(non_snake_case, non_camel_case_types)]

mod app;
mod graphics;
mod utils;

use crate::app::App;
use glfw::Context;

fn main() {
    let mut app = App::new();

    app.start();

    app.end()
}
