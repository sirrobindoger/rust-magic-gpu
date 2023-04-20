

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

pub trait Index {
    fn desc() -> wgpu::IndexFormat;
}

pub mod modelvertex;