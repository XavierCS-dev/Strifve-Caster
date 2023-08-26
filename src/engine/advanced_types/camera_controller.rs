use crate::engine::primitives::{
    matrix::Matrix4,
    quaternion::{self, Quaternion},
    vector::{Vector2, Vector3},
};

pub struct CameraController3D {
    position: Matrix4<f32>,
    rotation: Vector2<f32>,
    quat: Quaternion<f32>,
}

impl CameraController3D {
    pub fn new() -> Self {
        let position = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let quat = Quaternion::new(
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            0.0,
        );
        let rotation = Vector2 { x: 0.0, y: 0.0 };
        Self {
            position,
            rotation,
            quat,
        }
    }

    pub fn process_keyboard(&mut self, dx: f32, dy: f32) {
        self.position.set(3, 0, self.position.get(3, 1) + dx);
        self.position.set(3, 1, self.position.get(3, 1) + dy);
    }

    pub fn process_camera(&mut self, dx: f32, dy: f32) {
        self.rotation.x += dx;
        self.rotation.y += dy;
        if self.rotation.x > 180.0 {
            self.rotation.x = (self.rotation.x % 180.0) - 180.0;
        } else if self.rotation.x < -180.0 {
            self.rotation.x = (self.rotation.x % 180.0) + 180.0;
        }
        if self.rotation.y > 90.0 {
            self.rotation.y = 90.0;
        } else if self.rotation.y < -90.0 {
            self.rotation.y = -90.0;
        }
        println!("dx: {}, dy: {}", dx, dy);
        println!("y rot: {}", self.rotation.y);
        println!("x rot: {}", self.rotation.x);
    }

    pub fn build_transformation(&mut self) -> Matrix4<f32> {
        let magnitude = self.rotation.magnitude();
        let mut axis = Vector3 {
            x: -self.rotation.y,
            y: -self.rotation.x,
            z: 0.0,
        };
        axis.normalise();
        self.quat.set_axis(axis);
        self.quat.set_angle(magnitude);
        &self.quat.to_matrix() * &self.position
    }
}
