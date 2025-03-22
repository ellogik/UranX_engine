#![allow(non_snake_case, non_camel_case_types)]

mod app;
mod utils;

use glfw::Context;
use crate::app::App;

fn main() {
    let mut app = App::new();

    app.start();

    app.end()
}