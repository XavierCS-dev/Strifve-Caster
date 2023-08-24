use super::{matrix::Matrix4, quaternion::Quaternion, vector::Vector3};

pub struct Transformation3D {
    rotation: Quaternion<f32>,
    position: Matrix4<f32>,
    scale: Matrix4<f32>,
}

impl Transformation3D {
    pub fn new(position: Vector3<f32>, rotation: Quaternion<f32>, scale: f32) -> Self {
        let position = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [position.x, position.y, position.z, 1.0],
        ]);
        let scale = Matrix4::new([
            [scale, 0.0, 0.0, 0.0],
            [0.0, scale, 0.0, 0.0],
            [0.0, 0.0, scale, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Self {
            position,
            scale,
            rotation,
        }
    }

    pub fn position(&self) -> Vector3<f32> {
        Vector3 {
            x: self.position.get(3, 0),
            y: self.position.get(3, 1),
            z: self.position.get(3, 2),
        }
    }

    pub fn scale(&self) -> f32 {
        self.scale.get(0, 0)
    }

    pub fn rotation(&self) -> &Quaternion<f32> {
        &self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.rotation.set_angle(angle);
    }

    pub fn set_axis(&mut self, axis: Vector3<f32>) {
        self.rotation.set_axis(axis);
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale.set(0, 0, scale);
        self.scale.set(1, 1, scale);
        self.scale.set(2, 2, scale);
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position
            .set_column(3, [position.x, position.y, position.z, 1.0]);
    }

    pub fn to_raw(&self) -> [[f32; 4]; 4] {
        let scale_and_rotate = &self.scale * &self.rotation.to_matrix();
        (&self.position * &scale_and_rotate).to_raw()
    }
}

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

    pub fn rotation(&self) -> [[f32; 2]; 2] {
        self.rotation
    }

    pub fn scale(&self) -> [[f32; 2]; 2] {
        self.scale
    }
}
