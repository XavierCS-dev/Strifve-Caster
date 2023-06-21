use crate::engine::primitives::vertex::Vertex2D;
use crate::engine::primitives::{transformation::Transformation2D, vector::Vector2};
use crate::engine::texture::TEXTURE_IDS;
use crate::engine::traits::update_entity::UpdateEntity;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawEntity2D {
    position: [u32; 2],
    rotation: [[f32; 2]; 2],
    scale: [[f32; 2]; 2],
    origin: [u32; 2],
    vertex_position: [[f32; 2];4],
    tex_pos: [[f32; 2];4],
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
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 10]>())
                        as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 12]>())
                        as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 14]>())
                        as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 16]>())
                        as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 18]>())
                        as wgpu::BufferAddress,
                    shader_location: 12,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 20]>())
                        as wgpu::BufferAddress,
                    shader_location: 13,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 22]>())
                        as wgpu::BufferAddress,
                    shader_location: 14,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[u32; 2]>() + mem::size_of::<[f32; 24]>())
                        as wgpu::BufferAddress,
                    shader_location: 15,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct Entity2D {
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
        // Need to check the TextureMap instead.... may need to introduce RW lock to it or something...but that will come later when adding threaded code
        unsafe {
            let tex_ids = TEXTURE_IDS.lock().unwrap();
            if !tex_ids.contains(&tex_id) {
                panic!("Invalid texture id: {}", tex_id);
            }
            drop(tex_ids);
        }
        let vertices = [
            Vertex2D {
                position: [0.0, 0.0],
                tex_pos: [0.0, 0.0],
            },
            Vertex2D {
                position: [0.0, 0.0],
                tex_pos: [0.0, 0.0],
            },
            Vertex2D {
                position: [0.0, 0.0],
                tex_pos: [0.0, 0.0],
            },
            Vertex2D {
                position: [0.0, 0.0],
                tex_pos: [0.0, 0.0],
            },
        ];
        Self {
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
            vertex_position: self.vertices.map(|x| x.position),
            tex_pos: self.vertices.map(|x| x.tex_pos),
        }
    }

    pub fn texture_id(&self) -> u32 {
        self.tex_id
    }

    pub fn vertices(&self) -> &[Vertex2D; 4] {
        &self.vertices
    }
}

impl UpdateEntity for Entity2D {
    fn update(&mut self) {
        self.transformation.update(self.rotation, self.scale);
    }
}
