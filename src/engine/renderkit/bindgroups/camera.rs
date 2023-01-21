use wgpu::Buffer;


pub struct CameraBindGroup {
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl CameraBindGroup {
    pub fn new(device: &wgpu::Device) -> Self {
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("camera_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        count: None,
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: (false), min_binding_size: (None) }
                    }
                ]
            });
        CameraBindGroup {
            bind_group_layout: camera_bind_group_layout,
        }
    }

    pub fn create_bind_group(&self, camera_buffer: Buffer, device: &wgpu::Device) -> wgpu::BindGroup {
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("camera_bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding()
                }
            ]
        });
        camera_bind_group
    }
}