use wgpu::{BindGroupLayout, DepthStencilState, FragmentState, VertexState};

pub struct PipelineHandle {
    pub pipeline: wgpu::RenderPipeline,
    pub pipeline_layout: wgpu::PipelineLayout,
}

impl PipelineHandle {
    pub fn new(
        bind_group_layouts: &[&BindGroupLayout],
        vertex: VertexState,
        fragment: Option<FragmentState>,
        depth_stencil: Option<DepthStencilState>,
        device: &wgpu::Device,
    ) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex,
            fragment,
            depth_stencil,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        Self{
            pipeline,
            pipeline_layout,
        }
    }


}
