use super::shader_program::ShaderProgram;
use super::texture::Texture;
use gl;
use memoffset::offset_of;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture_coords: [f32; 2],
}

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    textures: Vec<Texture>,
    vertex_array: u32,
    vertex_buffer: u32,
    element_buffer: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let mut vertex_array = 0;
        let mut vertex_buffer = 0;
        let mut element_buffer = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::GenBuffers(1, &mut element_buffer);
        }

        let mesh = Self {
            vertices,
            indices,
            textures,
            vertex_array,
            vertex_buffer,
            element_buffer,
        };

        mesh.bind_Mesh();
        mesh
    }

    pub fn bind_Mesh(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<Vertex>()) as isize,
                self.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * mem::size_of::<u32>()) as isize,
                self.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, normal) as *const std::ffi::c_void,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, texture_coords) as *const std::ffi::c_void,
            );

            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self, shader_program: ShaderProgram) {
        unsafe {
            let mut shader_diffuse_normal: u32 = 1;
            let mut shader_specular_normal: u32 = 1;

            for (i, texture) in self.textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);

                let number = if texture.texture_type == "texture_diffuse" {
                    let num = shader_diffuse_normal;
                    shader_diffuse_normal += 1;
                    num
                } else if texture.texture_type == "texture_specular" {
                    let num = shader_specular_normal;
                    shader_specular_normal += 1;
                    num
                } else {
                    1
                };

                let name: String = format!("material{}{}", texture.texture_type, number);
                ShaderProgram::set_int(&shader_program, &name, i as f32);

                gl::BindTexture(gl::TEXTURE_2D, texture.texture_id);
            }

            gl::ActiveTexture(gl::TEXTURE0);

            gl::BindVertexArray(self.vertex_array);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }
}
