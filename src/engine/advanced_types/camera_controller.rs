use crate::engine::primitives::{
    matrix::Matrix4,
    quaternion::{self, Quaternion},
    vector::{Vector2, Vector3},
};

pub struct CameraController3D {
    position: Vector3<f32>,
    rotation: Vector2<f32>,
}

impl CameraController3D {
    pub fn new() -> Self {
        let position = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let rotation = Vector2 { x: 0.0, y: 0.0 };
        Self { position, rotation }
    }

    pub fn register_movement(&mut self, dx: f32, dy: f32) {
        self.rotation.x = (self.rotation.x + dx) % 360.0;
        // might wanna limit the range to +- 90.0 to stop weird 360 rot
        self.rotation.y = (self.rotation.y + dy) % 360.0;
    }

    pub fn build_transformation(&mut self) -> Matrix4<f32> {
        let magnitude = self.rotation.magnitude();
        let mut axis = self.rotation;
        axis.normalise();
        let axis = Vector3 {
            x: -axis.y,
            y: axis.x,
            z: 0.0,
        };
        let quat = Quaternion::new(axis, magnitude);
        quat.to_matrix()
    }
}
