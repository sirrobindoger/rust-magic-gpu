
use std::io::{BufReader, Cursor};

use wgpu::util::DeviceExt;

use crate::engine::resource::load_string;

use super::{texture::Texture, BindGroups, gpuhandle::GPUHandle, Renderable, buffers::modelvertex::ModelVertex};


pub struct Material {
    pub name : String,
    pub diffuse_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub material: usize,
}

pub struct Model {
    pub name: String,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl Material { 
    pub fn new(name: String, diffuse_texture: Texture, device: &wgpu::Device, bindgroups: &BindGroups) -> Self {
        let texture_bind_group = bindgroups.texture.create_bind_group(&diffuse_texture, device);

        Material {
            name,
            diffuse_texture,
            bind_group: texture_bind_group,
        }
    }
}

impl Renderable for Model {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass) {
        for mesh in &self.meshes {
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
        }
    }
}

impl Model {
    pub async fn new(filename: &str, gpu: &GPUHandle, bindgroups: &BindGroups) -> anyhow::Result<Model> {
        let obj_text = load_string(filename).await?;
        let obj_cursor = Cursor::new(obj_text);
        let mut obj_reader = BufReader::new(obj_cursor);

        let (models, obj_materials) = tobj::load_obj_buf_async(
            &mut obj_reader, 
            &tobj::LoadOptions{
                triangulate: true,
                single_index: true,
                ..Default::default()
            }, |p| async move {
                let mat_text = load_string(&p).await.unwrap();
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
            }).await?;
        
        let mut materials = Vec::new();

        for m in obj_materials? {
            let diffuse_texture = Texture::load_texture(&m.diffuse_texture, &gpu).await?;
            let material = Material::new(m.name, diffuse_texture, &gpu.device, &bindgroups);
            materials.push(material);
        }
        let meshes = models.into_iter()
            .map(|m| {
                let vertices = (0..m.mesh.positions.len() / 3)
                    .map(|i| ModelVertex {
                        position: [
                            m.mesh.positions[i * 3],
                            m.mesh.positions[i * 3 + 1],
                            m.mesh.positions[i * 3 + 2],
                        ],
                        tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                        normal: [
                            m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2],
                        ],
                    })
                    .collect::<Vec<_>>();
                let vertex_buffer = ModelVertex::new_buffer(&gpu, &vertices);
                let index_buffer = gpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&m.mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

                Mesh {
                    name: m.name,
                    vertex_buffer,
                    index_buffer,
                    index_count: m.mesh.indices.len() as u32,
                    material: m.mesh.material_id.unwrap_or(0),
                }
            })
            .collect::<Vec<_>>();

        Ok(Model {
            name: filename.to_string(),
            meshes,
            materials,
        })
    }
}