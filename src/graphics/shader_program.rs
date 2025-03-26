use gl;

#[derive(Debug, Clone)]
pub struct ShaderProgram {
    gl_shader_program: u32,
    gl_vertex_shader: u32,
    gl_fragment_shader: u32,
}

impl ShaderProgram {
    pub fn new(vertex_shader_path: &str, fradment_shader_path: &str) -> ShaderProgram {
        let gl_vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let gl_fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

        let program = ShaderProgram {
            gl_shader_program: 0,
            gl_vertex_shader,
            gl_fragment_shader,
        };

        program.compile_shader(vertex_shader_path, fradment_shader_path);
        program.create_shader_program();
        program.clear_shader();

        program
    }

    pub fn compile_shader(&self, vertex_shader_path: &str, fragment_shader_path: &str) {
        let vert_scr =
            std::fs::read_to_string(vertex_shader_path).expect("Failed to load vertex shader");
        let frag_scr =
            std::fs::read_to_string(fragment_shader_path).expect("Failed to load fragment shader");

        unsafe {
            let vert_cstr = std::ffi::CString::new(vert_scr).unwrap();
            let frag_cstr = std::ffi::CString::new(frag_scr).unwrap();

            gl::ShaderSource(
                self.gl_vertex_shader,
                1,
                &vert_cstr.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(self.gl_vertex_shader);

            gl::ShaderSource(
                self.gl_fragment_shader,
                1,
                &frag_cstr.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(self.gl_fragment_shader);
        }
    }

    pub fn create_shader_program(&self) {
        unsafe {
            gl::AttachShader(self.gl_shader_program, self.gl_vertex_shader);
            gl::AttachShader(self.gl_shader_program, self.gl_fragment_shader);
            gl::LinkProgram(self.gl_shader_program);
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.gl_shader_program);
        }
    }

    pub fn clear_shader(&self) {
        unsafe {
            gl::DeleteShader(self.gl_vertex_shader);
            gl::DeleteShader(self.gl_fragment_shader);
        }
    }

    pub fn gl_bind_uniform_mat4_fv(&self, name: &str, target: &[f32; 16]) {
        unsafe {
            let loc = gl::GetUniformLocation(
                self.gl_shader_program,
                std::ffi::CString::new(name).unwrap().as_ptr(),
            );

            gl::UniformMatrix4fv(loc, 1, gl::FALSE, target.as_ptr());
        }
    }

    pub fn set_int(&self, name: &str, value: f32) {
        unsafe {
            let loc = gl::GetUniformLocation(
                self.gl_shader_program,
                std::ffi::CString::new(name).unwrap().as_ptr(),
            );

            gl::Uniform1f(loc, value)
        }
    }
}
