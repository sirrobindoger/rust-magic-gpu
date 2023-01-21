use crate::engine::texture::Texture;



trait BindGroup {
    fn new(device: &wgpu::Device) -> Self;
    fn create_bind_group(&self, texture: &Texture, device: &wgpu::Device) -> wgpu::BindGroup;
}

mod camera;
mod texture;

pub struct BindGroups {
    pub camera: camera::CameraBindGroup,
    pub texture: texture::TextureBindGroup,
}

impl BindGroups {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            camera: camera::CameraBindGroup::new(device),
            texture: texture::TextureBindGroup::new(device),
        }
    }
}