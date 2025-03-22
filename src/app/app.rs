use glfw::Context;
use crate::app::Init;
use crate::app::init::{GLFWHolder, Settings};

pub struct App {
    pub is_end: bool,
    pub glfw_holder: GLFWHolder,
    pub settings: Settings
}

impl App {
    pub fn new() -> Self {
        let mut settings = Init::loadSettings().unwrap();
        let mut glfw_holder = Init::initGLFW(
            settings.graphics.window_width,
            settings.graphics.window_height,
            &(settings.manifest.name.clone() + " - UranX"),
            glfw::WindowMode::Windowed
        ).unwrap();
        Init::initOpenGL(&mut glfw_holder);


        Self {
            is_end: false,
            glfw_holder,
            settings
        }
    }

    pub fn start(&mut self) {
        while !self.is_end {
            unsafe {
                gl::ClearColor(0.5, 0.3, 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT)
            }

            self.glfw_holder.window.swap_buffers();
            self.glfw_holder.GLFW.poll_events();
            self.is_end = self.glfw_holder.window.should_close();
        }
    }

    pub fn end(self) {

    }
}