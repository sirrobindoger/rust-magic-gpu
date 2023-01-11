
use cgmath::{Matrix4, Vector3, Deg, perspective, vec3, InnerSpace, Point3, EuclideanSpace};
use winit::window::Window;

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    fov: f32,
    aspect: f32,
    znear: f32,
    zfar: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into()
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0_f32,
);

impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = perspective(Deg(self.fov), self.aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    pub fn new(window: &Window, config: &wgpu::SurfaceConfiguration) -> Self {
        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0,0.0,0.0).into(),
            up: Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fov: 45.0,
            znear: 0.1,
            zfar: 100.0
        };
        camera
    }
}