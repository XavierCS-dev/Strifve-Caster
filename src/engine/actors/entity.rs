use crate::engine::primitives::quaternion::Quaternion;
use crate::engine::primitives::transformation::Transformation3D;
use crate::engine::primitives::vector::Vector3;
use crate::engine::primitives::vertex::{Vertex2D, Vertex3D};
use crate::engine::primitives::{transformation::Transformation2D, vector::Vector2};
use crate::engine::traits::update_entity::UpdateEntity;
use rand::Rng;
use std::sync::Mutex;

pub static mut ENTITY_IDS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
pub struct RawEntity3D {
    position: [f32; 3],
    rotation: [[f32; 3]; 3],
    origin: [f32; 3],
    scale: [[f32; 3]; 3],
}

impl RawEntity3D {
    pub fn descriptor() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RawEntity3D>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 5,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 6,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 15]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 7,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 18]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 8,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 21]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 9,
                },
            ],
        }
    }
}

// TODO: Implement Entity3D

pub struct Entity3D {
    id: u32,
    texture_id: Option<u32>,
    // Position in world space
    position: Vector3<f32>,
    scale: f32,
    rotation: Quaternion<f32>,
    origin: Vector3<f32>,
    vertices: Vec<Vertex3D>,
    indices: Vec<u32>,
}

impl Entity3D {
    pub fn new(
        texture_id: Option<u32>,
        position: Vector3<f32>,
        scale: f32,
        rotation: Quaternion<f32>,
        origin: Vector3<f32>,
        vertices: Vec<Vertex3D>,
        indices: Vec<u32>,
    ) -> Self {
        // origin will need to be calculated instead of being a param
        let id = unsafe { Entity3D::create_id() };
        Self {
            id,
            texture_id,
            position,
            scale,
            rotation,
            origin,
            vertices,
            indices,
        }
    }

    pub fn rotation(&self) -> &Quaternion<f32> {
        &self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_texture(&mut self, texture_id: u32) {
        self.texture_id = Some(texture_id);
    }

    pub fn texture_id(&self) -> Option<u32> {
        self.texture_id
    }

    pub fn vertices(&self) -> (&Vec<Vertex3D>, &Vec<u32>) {
        (&self.vertices, &self.indices)
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vertex3D>, indices: Vec<u32>) {
        // Origin will need to be recalculated
        self.vertices = vertices;
        self.indices = indices;
    }

    unsafe fn create_id() -> u32 {
        let mut num = rand::thread_rng().gen_range(0..u32::MAX);
        let mut entity_ids = ENTITY_IDS.lock().unwrap();
        while entity_ids.contains(&num) {
            num = rand::thread_rng().gen_range(0..u32::MAX);
        }
        entity_ids.push(num);
        drop(entity_ids);
        num
    }

    pub fn to_raw(&self) -> RawEntity3D {
        RawEntity3D {
            position: self.position.to_raw(),
            scale: [
                [self.scale, 0.0, 0.0],
                [0.0, self.scale, 0.0],
                [0.0, 0.0, self.scale],
            ],
            origin: self.origin.to_raw(),
            rotation: self.rotation.to_raw(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawEntity2D {
    position: [u32; 2],
    rotation: [[f32; 2]; 2],
    scale: [[f32; 2]; 2],
    origin: [u32; 2],
}

impl RawEntity2D {
    pub fn descriptor() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RawEntity2D>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[u32; 2]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 2]>())
                        as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 6]>())
                        as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 8]>())
                        as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Uint32x2,
                },
            ],
        }
    }
}

pub struct Entity2D {
    id: u32,
    position: Vector2<u32>,
    rotation: f32,
    scale: f32,
    transformation: Transformation2D,
    origin: Vector2<u32>,
    tex_id: u32,
    vertices: [Vertex2D; 4], // vertices and texture coordinates change the shapes formed whenever the camera moves...
}

impl Entity2D {
    pub fn new(
        tex_id: u32,
        position: Vector2<u32>,
        rotation: f32,
        scale: f32,
        origin: Vector2<u32>,
    ) -> Self {
        let id = unsafe { Entity2D::create_id() };
        let vertices = [
            Vertex2D {
                position: [1.0, 1.0],
                tex_pos: [1.0, 1.0],
            },
            Vertex2D {
                position: [0.0, 1.0],
                tex_pos: [0.0, 1.0],
            },
            Vertex2D {
                position: [0.0, 0.0],
                tex_pos: [0.0, 0.0],
            },
            Vertex2D {
                position: [1.0, 0.0],
                tex_pos: [1.0, 0.0],
            },
        ];
        Self {
            id,
            position,
            rotation,
            scale,
            transformation: Transformation2D::new(rotation, scale),
            origin,
            tex_id,
            vertices,
        }
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn position(&self) -> Vector2<u32> {
        self.position
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn to_raw(&self) -> RawEntity2D {
        RawEntity2D {
            position: self.position.to_raw(),
            rotation: self.transformation.rotation(),
            scale: self.transformation.scale(),
            origin: self.origin.to_raw(),
        }
    }

    pub fn texture_id(&self) -> u32 {
        self.tex_id
    }

    pub fn vertices(&self) -> &[Vertex2D; 4] {
        &self.vertices
    }

    unsafe fn create_id() -> u32 {
        let mut num = rand::thread_rng().gen_range(0..u32::MAX);
        let mut entity_ids = ENTITY_IDS.lock().unwrap();
        while entity_ids.contains(&num) {
            num = rand::thread_rng().gen_range(0..u32::MAX);
        }
        entity_ids.push(num);
        drop(entity_ids);
        num
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Entity2D {
    fn drop(&mut self) {
        unsafe {
            let mut entity_ids = ENTITY_IDS.lock().unwrap();
            entity_ids.sort_unstable();
            let searched = entity_ids.binary_search(&self.id).unwrap();
            entity_ids.remove(searched);
            drop(entity_ids);
        }
    }
}

impl UpdateEntity for Entity2D {
    fn update(&mut self) {
        self.transformation.update(self.rotation, self.scale);
    }
}
