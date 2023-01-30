
use std::collections::HashMap;

use bytemuck::bytes_of;
use gpuhandle::GPUHandle;
use wgpu::{util::DeviceExt, include_wgsl};
use winit::window::Window;

use self::{pipelinehandle::PipelineHandle, bindgroups::BindGroups};

mod bindgroups;
mod gpuhandle;
mod texture;
mod pipelinehandle;
mod buffers;


pub trait Renderable {
    fn render(&self, render_pass: &mut wgpu::RenderPass);
}

pub struct RenderKit {
    pipeline: PipelineHandle,
    renderables: Vec<Box<dyn Renderable>>,
    bindgroups: BindGroups,
    pub gpu: GPUHandle
}

impl RenderKit {
    pub async fn new(window: &Window) -> Self {
        let gpu = GPUHandle::new(window).await;

        let bindgroups = BindGroups::new(&gpu.device);
        
        let shader = gpu.device.create_shader_module(include_wgsl!("../../shader.wgsl"));
        let vertex_state = wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        };

        let fragment_state = wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: gpu.config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        };

        let pipeline = PipelineHandle::new(
            &[&bindgroups.texture.bind_group_layout, &bindgroups.camera.bind_group_layout],
            vertex_state,
            Some(fragment_state),
            None,
            &gpu.device,
        );

        RenderKit {
            renderables: Vec::new(),
            bindgroups,
            pipeline,
            gpu
        }
    }

    pub fn insert_renderable(&mut self, renderable: Box<dyn Renderable>) {
        self.renderables.push(renderable);
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.gpu.resize(size);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.gpu.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: true,
                    }
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline.pipeline);
            

            
        }

        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    fn create_vertex_buffer(&self, data: &[u8]) {
        let buffer = self.gpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: data,
            usage: wgpu::BufferUsages::VERTEX,
        });
    }

    fn create_index_buffer(&self, data: &[u8]) {
        let buffer = self.gpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: data,
            usage: wgpu::BufferUsages::INDEX,
        });
    }
    
}