#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub tex_pos: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub tex_pos: [f32; 3],
}
