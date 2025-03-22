use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ERRUranXError {
    CANNOT_INIT_LIBRARY(Library),
    CANNOT_INIT_SETTINGS,
    RUNTIME_LIBRARY_ERROR(Library),
}

#[derive(Debug)]
pub enum Library {
    GLFW,
    OPEN_GL
}

impl Display for ERRUranXError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UranX can't work because We have a critical error.")
    }
}

impl Error for ERRUranXError {}