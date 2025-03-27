use crate::app::init::{GLFWHolder, Settings};
use crate::app::Init;
use crate::graphics::camera::Camera;
use crate::graphics::model::Model;
use crate::graphics::shader_program::ShaderProgram;
use gl::SHADER_SOURCE_LENGTH;
use glfw::Context;

pub struct App {
    pub is_end: bool,
    pub glfw_holder: GLFWHolder,
    pub settings: Settings,
    camera: Camera,
    shader: ShaderProgram,
    model: Model,
}

impl App {
    pub fn new() -> Self {
        let settings = Init::loadSettings().unwrap();
        let mut glfw_holder = Init::initGLFW(
            settings.graphics.window_width,
            settings.graphics.window_height,
            &(settings.manifest.name.clone() + " - UranX"),
            glfw::WindowMode::Windowed,
        )
        .unwrap();
        Init::initOpenGL(&mut glfw_holder);

        glfw_holder.window.make_current();
        glfw_holder.window.set_cursor_pos_polling(true);

        let camera = Camera::new(
            [0.0, 0.0, 3.0],
            [0.0, 1.0, 0.0],
            0.3,
            settings.graphics.window_width as f32,
            settings.graphics.window_height as f32,
        );

        let model = Model::new("res/objects/backpack.obj");
        let shader = ShaderProgram::new("res/shaders/shd.vert", "res/shaders/shd.frag");

        Self {
            is_end: false,
            glfw_holder,
            settings,
            camera,
            shader,
            model,
        }
    }

    pub fn start(&mut self) {
        while !self.is_end {
            unsafe {
                gl::Enable(gl::DEPTH_TEST);
                gl::ClearColor(0.1, 0.1, 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            self.shader.use_program();

            let projection = Camera::perspective(
                45.0,
                self.settings.graphics.window_height as f32
                    / self.settings.graphics.window_height as f32,
                0.1,
                100.0,
            );

            let view = Camera::look_at(
                self.camera.gl_camera_pos,
                [
                    self.camera.gl_camera_pos[0] + self.camera.gl_camera_front[0],
                    self.camera.gl_camera_pos[1] + self.camera.gl_camera_front[1],
                    self.camera.gl_camera_pos[2] + self.camera.gl_camera_front[2],
                ],
                self.camera.gl_camera_up,
            );

            let mut model = Camera::identity_matrix();
            model = Camera::translate(model, [0.0, 0.0, 3.0]);
            model = Camera::scale(model, [1.0, 1.0, 1.0]);

            self.shader
                .gl_bind_uniform_mat4_fv("projection", &projection);
            self.shader.gl_bind_uniform_mat4_fv("view", &view);
            self.shader.gl_bind_uniform_mat4_fv("model", &model);

            println!("drewing");
            self.model.draw(self.shader.gl_shader_program);

            self.glfw_holder.window.swap_buffers();
            self.glfw_holder.GLFW.poll_events();
            self.is_end = self.glfw_holder.window.should_close();
        }
    }

    pub fn end(self) {}
}
