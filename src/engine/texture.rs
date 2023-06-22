use crate::engine::advanced_types::texture_vecs::Texture2DMap;
use crate::engine::primitives::vertex::Vertex2D;
use anyhow::Result;
use rand::Rng;
use std::{collections::HashSet, fs::File, io, io::BufReader, io::Read, sync::Mutex};

use image::GenericImageView;

/// Contains a list of texture IDs. Only intended to be used by advanced users.
pub static mut TEXTURE_IDS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

#[derive(Debug)]
pub struct Texture2D {
    id: u32,
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
        // POSSIBLY CHANGE CREATING TEXTURE TO CREATE A TEXTURE AND ADD IT TO THE HASHMAP, RETURN ID INSTEAD OF SELF
        let id = unsafe { Texture2D::create_id() };
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
            format: wgpu::TextureFormat::Rgba8Unorm,
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
            id,
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

    pub fn id(&self) -> u32 {
        self.id
    }

    /// Creates and adds and ID to TextureIDs. Can cause TextureIDs to leak IDs.
    unsafe fn create_id() -> u32 {
        let mut num = rand::thread_rng().gen_range(0..std::u32::MAX);
        let mut tex_ids = TEXTURE_IDS.lock().unwrap();
        while tex_ids.contains(&num) {
            num = rand::thread_rng().gen_range(0..std::u32::MAX);
        }
        tex_ids.push(num);
        drop(tex_ids);
        num
    }
}
