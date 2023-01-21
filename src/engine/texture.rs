use image::GenericImageView;
use anyhow::*;

use super::renderkit::Renderable;


pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Renderable for Texture {
    fn register_bind_groups(&self, device: wgpu::Device) -> wgpu::BindGroupLayout {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
            texture_bind_group_layout
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        todo!()
    }
}

impl Texture {
    fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let diffuse_rgba = img.to_rgba8();

        use image::GenericImageView;

        let dim = img.dimensions();

        // lets create a texture to load our happy tree image

        let texture_size = wgpu::Extent3d {
            width: dim.0,
            height: dim.1,
            depth_or_array_layers: 1,
        };

        // make texture
        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor{ 
                label: Some("diffuse_texture_happytree :D"),
                // ALL textures are stored as 3D, we represent our 2d Texture
                // bysetting it's deapth to 1.
                size: texture_size,
                mip_level_count: 1, // we'll tal;k about t his latr
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // MOST images are stored as sRGB, let's tell the GPU that
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                // TEXTURE_BINDING tells the GPU that we want to use this texture in shaders!
                // COPY_DST means we wantto copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            }
        );

        // lets load our texture into the GPU
        queue.write_texture(
            wgpu::ImageCopyTexture{
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // pixeldata
            &diffuse_rgba,
            // Layout of texxture
            wgpu::ImageDataLayout{
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4*dim.0),
                rows_per_image: std::num::NonZeroU32::new(dim.1)
            },
            texture_size
        );

        let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let diffuse_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("E"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,

                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );


        Ok(Self {
            texture: diffuse_texture,
            view: diffuse_texture_view,
            sampler: diffuse_sampler,
        })
    }
}