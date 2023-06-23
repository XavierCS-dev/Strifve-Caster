use crate::engine::actors::entity::{Entity2D, RawEntity2D};
use crate::engine::primitives::vertex::Vertex2D;
use crate::engine::texture::Texture2D;
use wgpu::util::DeviceExt;

// The idea of Batch2D is to collect all the raw data from the users, and store buffers, for each batch of entities.
// This allows an easily modifiable group of entities with the same texture to be drawn together.
// Having a Batch struct allows the use of separate buffers for each batch, without having to fight
// the borrow checker with render_pass (doesn't live long enough as render_pass keeps borrow after draw())
// It also avoids complicated and messy code with having one large buffer for all vertices, and one for all
// entity data etc.
// The trade-off is performance, with memory being allocated n times more often, where n is the number
// of Batches.
// However if a batch doesn't update, the buffer won't need to be reallocated. In the case of this engine,
// this will be very rare due to the nature of raycasters, and may only apply to sprites.

pub struct Batch2D {
    id: u32,
    entity_data: Vec<RawEntity2D>,
    vertex_data: Vec<Vertex2D>,
    indices: Vec<u16>,
    texture: Texture2D,
    entity_buffer: Option<wgpu::Buffer>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
}

impl Batch2D {
    pub fn new(
        texture_path: &str,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let texture = Texture2D::new(texture_path, queue, device, bind_group_layout);
        let entity_data = Vec::new();
        let vertex_dara = Vec::new();
        let indices = Vec::new();
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
        self.entity_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Entity Buffer"),
                contents: bytemuck::cast_slice(&self.entity_data),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
        self.vertex_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Entity Buffer"),
                contents: bytemuck::cast_slice(&self.vertex_data),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
        // TODO: Implement index vec and buffer creation, index buffer / vec should only be updated if no# of entities changes
        // TODO: Implement vertex buffer creation, should be adding same 6 vertices repeated n number of times where n is the number of entities
        todo!();
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
