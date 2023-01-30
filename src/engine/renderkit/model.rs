
use super::{texture::Texture, BindGroups};


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
    pub material: Material,
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

impl Mesh {
    pub fn new(name: String, vertex_buffer: wgpu::Buffer, index_buffer: wgpu::Buffer, index_count: u32, material: Material) -> Self {
        Mesh {
            name,
            vertex_buffer,
            index_buffer,
            index_count,
            material,
        }
    }
}

impl Renderable for Model {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        todo!()
    }
}

impl Model {
    pub fn new(filename: &str) -> Self {

        todo!();
    }
}