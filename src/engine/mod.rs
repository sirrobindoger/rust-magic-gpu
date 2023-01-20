use std::collections::HashMap;

use gpuhandle::GPUHandle;
use wgpu::util::DeviceExt;
use winit::window::Window;

mod gpuhandle;
mod shaderhandle;

pub trait Renderable {
    fn render(&self, kit: &mut RenderKit);
}

pub struct RenderKit {
    bind_groups: HashMap<String, wgpu::BindGroup>,
    bind_group_layouts: HashMap<String, wgpu::BindGroupLayout>,
    pipelines: HashMap<String, wgpu::RenderPipeline>,
    gpu: GPUHandle
}

impl RenderKit {
    pub async fn new(window: &Window) -> Self {
        RenderKit {
            bind_groups: HashMap::new(),
            bind_group_layouts: HashMap::new(),
            pipelines: HashMap::new(),
            gpu: GPUHandle::new(window).await
        }
    }

    pub fn create_bind_group_layout(&mut self, name: &str, entries: &[wgpu::BindGroupLayoutEntry]) {
        let layout = self.gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(name),
            entries,
        });
        self.bind_group_layouts.insert(name.to_string(), layout);
    }

    // create a pipeline from a shader, file name
    fn create_pipeline(&mut self, 
        name: &str, 
        bind_group_layouts_names: &Vec<String>, 
        shader: &str, 
        buffers: &[wgpu::VertexBufferLayout],
    ) {
        
        let file_data = std::fs::read_to_string(shader).unwrap();

        let shader = self.gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(file_data.into()),
        });
        
        let render_pipeline_layout = self.gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("Render Pipeline Layout"),
            // vec to slice
            bind_group_layouts: &bind_group_layouts_names.iter().map(|name| self.bind_group_layouts.get(name).unwrap()).collect::<Vec<&wgpu::BindGroupLayout>>(),
            push_constant_ranges: &[],
        });

        let render_pipeline = self.gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &buffers,
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.gpu.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
        }); 
        self.pipelines.insert(name.to_string(), render_pipeline);
    }

    fn create_vertex_buffer(&self, data: &[u8]) {
        let buffer = self.gpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: data,
            usage: wgpu::BufferUsages::VERTEX,
        });
    }
}