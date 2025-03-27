use super::mesh::Mesh;
use super::mesh::Vertex;
use super::texture::Texture;
use std::collections::HashMap;
use std::string::String;
use tobj;

#[derive(Debug)]
pub struct Model {
    meshes: Vec<Mesh>,
    texture_loader: HashMap<String, Texture>,
    directory: String,
}

impl Model {
    pub fn new(path: &str) -> Model {
        let mut model = Model {
            meshes: Vec::new(),
            texture_loader: HashMap::new(),
            directory: String::new(),
        };

        if let Some(parent) = std::path::Path::new(path).parent() {
            model.directory = parent.to_string_lossy().to_string();
        };

        model.load_model(path);
        model
    }

    fn load_model(&mut self, path: &str) {
        let (models, materials) =
            tobj::load_obj(path, &tobj::LoadOptions::default()).expect("Failed to load .obj");

        for model in models {
            self.process_mesh(&model);
        }

        if let Ok(materials) = materials {
            for material in materials {
                if !material.diffuse_texture.is_empty() {
                    let texture_path = format!("{}/{}", self.directory, material.diffuse_texture);
                    println!("[DEBUG] Loading texture: {}", texture_path);
                    let texture = Texture::load(&texture_path, "diffuse".to_string());
                    self.texture_loader
                        .insert(material.diffuse_texture.clone(), texture);
                }
            }
        }
    }

    fn process_mesh(&mut self, model: &tobj::Model) {
        let mesh = &model.mesh;
        let mut vertices = Vec::new();
        let indices = mesh.indices.clone();
        let mut texture = Vec::new();

        for i in 0..mesh.positions.len() / 3 {
            let position = [
                mesh.positions[i * 3],
                mesh.positions[i * 3 + 1],
                mesh.positions[i * 3 + 2],
            ];

            let normal = if !mesh.normals.is_empty() && (i * 3 + 2) < mesh.normals.len() {
                [
                    mesh.normals[i * 3],
                    mesh.normals[i * 3 + 1],
                    mesh.normals[i * 3 + 2],
                ]
            } else {
                [0.0, 0.0, 0.0]
            };

            let texture_coords = if !mesh.texcoords.is_empty() {
                [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]]
            } else {
                [0.0, 0.0]
            };

            println!(
                "[DEBUG] Vertex {}: Pos({:?}), Normal({:?}), UV({:?})",
                i, position, normal, texture_coords
            );

            vertices.push(Vertex {
                position,
                normal,
                texture_coords,
            });
        }

        if let Some(material_id) = mesh.material_id {
            if let Some(material) = self
                .texture_loader
                .get(&format!("{}/{}", self.directory, material_id))
            {
                texture.push(material.clone());
            }
        }

        let mesh = Mesh::new(vertices, indices, texture);
        self.meshes.push(mesh);
    }

    pub fn draw(&self, shader_program: u32) {
        for mesh in &self.meshes {
            mesh.draw(shader_program);
        }
    }
}
