use crate::engine::primitives::vertex::Vertex2D;
use anyhow::Result;
use rand::Rng;
use std::{collections::HashSet, fs::File, io, io::BufReader, io::Read, sync::Mutex};

use image::GenericImageView;

#[derive(Debug)]
pub struct Texture2D {
    diffuse_texture: wgpu::Texture,
    sampler: wgpu::Sampler,
    view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
    rgba_buffer: image::RgbaImage,
    dimensions: wgpu::Extent3d,
}

impl Texture2D {
    pub fn new(
        file_path: &str,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<Self> {
        let mut byte_vec: Vec<u8> = Vec::new();
        let file = File::open(file_path)?;
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_end(&mut byte_vec)?;
        let bytes = byte_vec.as_slice();
        let image = image::load_from_memory(bytes).unwrap();
        let rgba_buffer = image.to_rgba8();
        let dimensions = image.dimensions();
        let texture_dimensions = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Diffuse Texture"),
            size: texture_dimensions,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba_buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                // TODO: Using std::num::NonZeroU32 has seem to broke..use regular U32 for now...
                bytes_per_row: Some(4 * texture_dimensions.width),
                rows_per_image: Some(texture_dimensions.height),
            },
            texture_dimensions,
        );

        let texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Sampler"),
            // change back to clamp to edge
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group Layout"),
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler),
                },
            ],
        });
        Ok(Self {
            diffuse_texture,
            sampler: texture_sampler,
            view: texture_view,
            bind_group,
            rgba_buffer,
            dimensions: texture_dimensions,
        })
    }

    pub fn texture(&self) -> &wgpu::Texture {
        &self.diffuse_texture
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn rgba_buffer(&self) -> &image::RgbaImage {
        &self.rgba_buffer
    }

    pub fn dimensions(&self) -> wgpu::Extent3d {
        self.dimensions
    }
}
