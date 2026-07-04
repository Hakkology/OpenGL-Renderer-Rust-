use crate::shaders::Texture;
use image::GenericImageView;

pub mod formats;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetType {
    Obj,
    Png,
    Jpg,
    Unknown,
}

impl AssetType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "obj" => AssetType::Obj,
            "png" => AssetType::Png,
            "jpg" | "jpeg" => AssetType::Jpg,
            _ => AssetType::Unknown,
        }
    }
}

pub struct AssetImporter;

impl AssetImporter {
    pub fn load_texture(path: &str) -> Result<Texture, String> {
        let img = image::open(path).map_err(|e| format!("Failed to open image {}: {}", path, e))?;
        let (width, height) = img.dimensions();
        let data = img.to_rgba8();

        println!("Loaded texture from: {}, size: {}x{}", path, width, height);

        Ok(Texture::new(width, height, &data, gl::RGBA))
    }

    pub fn load_model(path: &str) -> Result<crate::scene::model::Model, String> {
        use crate::scene::model::Mesh;

        let (models, _materials) = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )
        .map_err(|e| format!("Failed to load model {}: {}", path, e))?;

        let mut meshes = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            let mut vertices = Vec::new();
            let num_vertices = mesh.positions.len() / 3;

            // Generate normals if they are missing
            let mut normals = mesh.normals.clone();
            if normals.is_empty() {
                normals = vec![0.0; mesh.positions.len()];
                for chunk in mesh.indices.chunks_exact(3) {
                    let i0 = chunk[0] as usize;
                    let i1 = chunk[1] as usize;
                    let i2 = chunk[2] as usize;

                    let p0 = glam::Vec3::new(mesh.positions[3 * i0], mesh.positions[3 * i0 + 1], mesh.positions[3 * i0 + 2]);
                    let p1 = glam::Vec3::new(mesh.positions[3 * i1], mesh.positions[3 * i1 + 1], mesh.positions[3 * i1 + 2]);
                    let p2 = glam::Vec3::new(mesh.positions[3 * i2], mesh.positions[3 * i2 + 1], mesh.positions[3 * i2 + 2]);

                    let edge1 = p1 - p0;
                    let edge2 = p2 - p0;
                    let n = edge1.cross(edge2);

                    for &i in &[i0, i1, i2] {
                        normals[3 * i] += n.x;
                        normals[3 * i + 1] += n.y;
                        normals[3 * i + 2] += n.z;
                    }
                }

                // Normalize accumulated vertex normals
                for i in 0..num_vertices {
                    let n = glam::Vec3::new(normals[3 * i], normals[3 * i + 1], normals[3 * i + 2]);
                    if n.length_squared() > 0.0 {
                        let n = n.normalize();
                        normals[3 * i] = n.x;
                        normals[3 * i + 1] = n.y;
                        normals[3 * i + 2] = n.z;
                    }
                }
            }

            for i in 0..num_vertices {
                // Position
                vertices.push(mesh.positions[3 * i]);
                vertices.push(mesh.positions[3 * i + 1]);
                vertices.push(mesh.positions[3 * i + 2]);

                // TexCoords (Flip Y coordinate for OpenGL UV orientation)
                if !mesh.texcoords.is_empty() {
                    vertices.push(mesh.texcoords[2 * i]);
                    vertices.push(1.0 - mesh.texcoords[2 * i + 1]);
                } else {
                    vertices.push(0.0);
                    vertices.push(0.0);
                }

                // Normals
                vertices.push(normals[3 * i]);
                vertices.push(normals[3 * i + 1]);
                vertices.push(normals[3 * i + 2]);
            }

            meshes.push(Mesh::new(&vertices, &mesh.indices));
        }

        println!("Loaded model: {}, meshes: {}", path, meshes.len());

        Ok(crate::scene::model::Model::new(meshes))
    }
}
