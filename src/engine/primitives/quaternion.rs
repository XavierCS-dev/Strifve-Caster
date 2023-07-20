use std::f32::consts::PI;

use crate::engine::primitives::vector::Vector3;

pub struct Quaternion {
    angle: f32,
    axis: Vector3<f32>,
}

impl Quaternion {
    pub fn new(axis: Vector3<f32>, angle: f32) -> Self {
        Self { angle, axis }
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    pub fn set_axis(&mut self, axis: Vector3<f32>) {
        self.axis = axis;
    }

    pub fn axis(&self) -> &Vector3<f32> {
        &self.axis
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn to_raw(&self) -> [[f32; 3]; 3] {
        let angle = (self.angle * PI) / 180.0;
        [
            [
                1.0 - 2.0 * (self.axis.y.powi(2) + self.axis.z.powi(2)),
                2.0 * (self.axis.x * self.axis.y + self.axis.z * angle),
                2.0 * (self.axis.x * self.axis.z - self.axis.y * angle),
            ],
            [
                2.0 * (self.axis.x * self.axis.z - self.axis.z * angle),
                1.0 - 2.0 * (self.axis.x.powi(2), self.axis.z.powi(2)),
                2.0 * (self.axis.y * self.axis.z + self.axis.x * angle),
            ],
            [
                2.0 * (self.axis.x * self.axis.z + self.axis.y * angle),
                2.0 * (self.axis.y * self.axis.z - self.axis.x * angle),
                1.0 - 2.0 * (self.axis.x.powi(2) + self.axis.y.powi(2)),
            ],
        ]
    }
}
