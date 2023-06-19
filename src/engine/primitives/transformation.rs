#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Transformation2D {
    rotation: [[f32; 2]; 2],
    scale: [[f32; 2]; 2],
}

impl Transformation2D {
    pub fn new(rotation: f32, scale: f32) -> Self {
        let rotation = [
            [rotation.cos(), -(rotation.sin())],
            [rotation.sin(), rotation.cos()],
        ];

        let scale = [[scale, 0.0], [0.0, scale]];

        Self { rotation, scale }
    }

    pub fn update(&mut self, rotation: f32, scale: f32) {
        self.rotation = [
            [rotation.cos(), -(rotation.sin())],
            [rotation.sin(), rotation.cos()],
        ];
        self.scale[0][0] = scale;
        self.scale[1][1] = scale;
    }

    pub fn rotation(&self) -> [[f32;2];2] {
        self.rotation
    }

    pub fn scale(&self) -> [[f32;2];2] {
        self.scale
    }
}
