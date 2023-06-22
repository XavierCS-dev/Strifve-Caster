use wgpu::util::DeviceExt;
use crate::engine::actors::entity::{Entity2D, RawEntity2D};
use crate::engine::primitives::vertex::Vertex2D;
use crate::engine::texture::Texture2D;

pub struct Batch2D {
    id: u32,
    entity_data: Vec<RawEntity2D>,
    vertex_data: Vec<Vertex2D>,
    texture: Texture2D,
    entity_buffer: Option<wgpu::Buffer>,
    vertex_buffer: Option<wgpu::Buffer>,
}

impl Batch2D {
    pub fn new(texture_path: &str, queue: &wgpu::Queue, device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let texture = Texture2D::new(texture_path, queue, device, bind_group_layout);
        let entity_data = Vec::new();
        let entity_buffer = None;
        let vertex_buffer = None;

        // TODO: Implement ID Creation
    }

    pub fn update(&mut self, entities: Vec<&Entity2D>, device: &wgpu::Device) {
        for entity in entities {
            self.entity_data.push(entity.to_raw());
            for vertex in entity.vertices() {
                self.vertex_data.push(*vertex);
            }
        }
        self.entity_buffer = Some (
            device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Entity Buffer"),
                    contents: bytemuck::cast_slice(&self.entity_data),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            )
        );
        self.vertex_buffer = Some (
            device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Entity Buffer"),
                    contents: bytemuck::cast_slice(&self.vertex_data),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            )
        );
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.texture.bind_group()
    }

    pub fn vertex_buffer(&self) -> Option<&wgpu::Buffer> {
        match &self.vertex_buffer {
            Some(b) => Some(&b),
            _ => None,
        }
    }

    pub fn entity_buffer(&self) -> Option<&wgpu::Buffer> {
        match &self.entity_buffer {
            Some(b) => Some(&b),
            _ => None,
        }
    }
}