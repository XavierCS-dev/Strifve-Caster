use crate::engine::actors::entity::{Entity2D, RawEntity2D};
use crate::engine::primitives::vertex::Vertex2D;
use crate::engine::texture::Texture2D;
use num_traits::abs;
use rand::Rng;
use std::sync::Mutex;
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

pub static mut BATCH_IDS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

pub struct Batch2D {
    id: u32,
    entity_data: Vec<RawEntity2D>,
    vertex_data: Vec<Vertex2D>,
    indices: Vec<u16>,
    texture: Texture2D,
    entity_buffer: Option<wgpu::Buffer>,
    entity_count: usize,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    // TEMP
    updated: bool,
}

impl Batch2D {
    pub fn new(
        texture_path: &str,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let id = unsafe { Batch2D::create_id() };
        let texture = Texture2D::new(texture_path, queue, device, bind_group_layout)
            .expect(format!("Could not find image {}", texture_path).as_str());
        let entity_data = Vec::new();
        let vertex_data = Vec::new();
        let indices = Vec::new();
        let entity_buffer = None;
        let entity_count: usize = 0;
        let vertex_buffer = None;
        let index_buffer = None;

        Self {
            id,
            entity_data,
            vertex_data,
            indices,
            texture,
            entity_buffer,
            entity_count,
            vertex_buffer,
            index_buffer,
            updated: false,
        }
    }

    pub fn update(&mut self, entities: &Vec<Entity2D>, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut recreate_index = false;
        if entities.len() != self.entity_count {
            recreate_index = true;
        }
        self.entity_data.clear();
        self.vertex_data.clear();
        for entity in entities {
            self.entity_data.push(entity.to_raw());
            for vertex in entity.vertices() {
                self.vertex_data.push(*vertex);
            }
        }

        if !self.updated {
            self.entity_buffer = Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Entity Buffer"),
                    contents: bytemuck::cast_slice(&self.entity_data),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            );
            self.vertex_buffer = Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&self.vertex_data),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            );
            self.updated = true;
        } else {
            queue.write_buffer(&self.entity_buffer.as_ref().unwrap(), 0, bytemuck::cast_slice(&self.entity_data));
            queue.write_buffer(&self.vertex_buffer.as_ref().unwrap(), 0, bytemuck::cast_slice(&self.vertex_data));
        }

        if recreate_index {
            let mut difference = entities.len() as i32 - self.entity_count as i32;
            self.entity_count += difference as usize;
            if difference < 0 {
                difference = abs(difference);
                for _ in 0..difference {
                    for _ in 0..6 {
                        self.indices
                            .pop()
                            .expect("Tried to pop more entities off batch than exists");
                    }
                }
            } else if difference > 0 {
                for _ in 0..difference {
                    self.indices.push(0);
                    self.indices.push(1);
                    self.indices.push(2);
                    self.indices.push(0);
                    self.indices.push(2);
                    self.indices.push(3);
                }
            } else {
                panic!("number of entities didn't change but still entered update index");
            }
            self.index_buffer = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&self.indices),
                    usage: wgpu::BufferUsages::INDEX,
                },
            ));
            self.entity_count = entities.len();
        }
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

    pub fn index_buffer(&self) -> Option<&wgpu::Buffer> {
        match &self.index_buffer {
            Some(b) => Some(&b),
            _ => None,
        }
    }

    pub fn entity_count(&self) -> u32 {
        self.entity_count as u32
    }

    unsafe fn create_id() -> u32 {
        let mut num = rand::thread_rng().gen_range(0..u32::MAX);
        let mut batch_ids = BATCH_IDS.lock().unwrap();
        while batch_ids.contains(&num) {
            num = rand::thread_rng().gen_range(0..u32::MAX);
        }
        batch_ids.push(num);
        drop(batch_ids);
        num
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Batch2D {
    fn drop(&mut self) {
        unsafe {
            let mut batch_ids = BATCH_IDS.lock().unwrap();
            batch_ids.sort_unstable();
            let searched = batch_ids.binary_search(&self.id).unwrap();
            batch_ids.remove(searched);
            drop(batch_ids);
        }
    }
}
