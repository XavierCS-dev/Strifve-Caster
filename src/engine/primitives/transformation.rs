pub struct Transformation2D {
    rotation: [[f32; 2]; 2],
    rotation_deg: f32,
    scale: [[f32; 2]; 2],
}

impl Transformation2D {
    pub fn new(rotation: f32, scale: f32) -> Self {
        let rotation_deg = rotation;
        let rotation = [
            [rotation.cos(), -(rotation.sin())],
            [rotation.sin(), rotation.cos()],
        ];

        let scale = [[scale, 0.0], [0.0, scale]];

        Self {
            rotation,
            rotation_deg,
            scale,
        }
    }

    pub fn scale(&self) -> f32 {
        self.scale[0][0]
    }

    pub fn rotation(&self) -> f32 {
        self.rotation_deg
    }

    pub fn update(&mut self, rotation: f32, scale: f32) {
        self.rotation_deg = rotation;
        self.rotation = [
            [rotation.cos(), -(rotation.sin())],
            [rotation.sin(), rotation.cos()],
        ];
        self.scale[0][0] = scale;
        self.scale[1][1] = scale;
    }
}
