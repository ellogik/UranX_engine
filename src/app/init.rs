use serde_derive::Deserialize;
use std::fs;
use crate::utils::errors::{ERRUranXError, Library};

pub struct Init;

impl Init {
    pub fn initGLFW(width: u32, height: u32, title: &str, window_mode: glfw::WindowMode) -> Result<GLFWHolder, ERRUranXError> {
        let mut GLFW = match glfw::init(glfw::fail_on_errors) {
            Ok(it) => it,
            Err(_) => return Err(ERRUranXError::CANNOT_INIT_LIBRARY(Library::GLFW))
        };

        let WINDOW = match GLFW
            .create_window(width, height, title, window_mode) {
            Some(it) => it,
            None => return Err(ERRUranXError::RUNTIME_LIBRARY_ERROR(Library::GLFW))
        };

        Ok(GLFWHolder { GLFW, window: WINDOW.0, events: WINDOW.1} )
    }

    pub fn initOpenGL(glfw_holder: &mut GLFWHolder) {
        gl::load_with(|s| { glfw_holder.window.get_proc_address(s) as *const _ })
    }

    pub fn loadSettings() -> Result<Settings, ERRUranXError> {
        match toml::from_str::<Settings>(
            fs::read_to_string("res/settings.toml").unwrap().as_str()
        ) {
            Ok(it) => Ok(it),
            Err(_) => Err(ERRUranXError::CANNOT_INIT_SETTINGS)
        }
    }
}

pub struct GLFWHolder {
    pub GLFW: glfw::Glfw,
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>
}

#[derive(Deserialize)]
pub struct Settings {
    pub manifest: ManifestSettings,
    pub graphics: GraphicsSettings
}

#[derive(Deserialize)]
pub struct GraphicsSettings {
    pub window_height: u32,
    pub window_width: u32,
    pub is_fullscreen: bool
}

#[derive(Deserialize)]
pub struct ManifestSettings {
    pub name: String,
    pub version: String
}