use core::num;
use std::f32::consts::PI;

use num_traits::{Float, NumCast};

use crate::engine::primitives::vector::Vector3;

use super::matrix::Matrix4;

pub struct Quaternion<T>
where
    T: num_traits::Num + NumCast + Copy,
{
    angle: T,
    axis: Vector3<T>,
    unit: bool,
}

impl<T> Quaternion<T>
where
    T: NumCast + Copy + num_traits::Num + Float,
{
    /*
        axis: the axis to rotate around
        angle: the angle to rotate
        unit: whether the quaternion is a unit quaternion
            (I am worried about floating point inaccuracies causing scaling)
    */
    pub fn new(axis: Vector3<T>, angle: T, unit: bool) -> Self {
        Self { angle, axis, unit }
    }

    pub fn set_rotation(&mut self, axis: Vector3<T>, angle: T) {
        self.axis = axis;
        self.angle = angle;
    }

    pub fn axis(&self) -> &Vector3<T> {
        &self.axis
    }

    pub fn angle(&self) -> T {
        self.angle
    }

    pub fn to_matrix(&self) -> Matrix4<f32> {
        Matrix4::new(self.to_raw())
    }

    pub fn to_raw(&self) -> [[f32; 4]; 4] {
        let w = (self.angle.to_f32().unwrap().to_radians() / 2.0).cos();
        let angle = self.angle.to_f32().unwrap().to_radians();
        let x = self.axis.x.to_f32().unwrap() * (angle / 2.0).sin();
        let y = self.axis.y.to_f32().unwrap() * (angle / 2.0).sin();
        let z = self.axis.z.to_f32().unwrap() * (angle / 2.0).sin();
        // https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation
        [
            [
                w.powi(2) + x.powi(2) - y.powi(2) - z.powi(2),
                2.0 * x * y + 2.0 * w * z,
                2.0 * x * z - 2.0 * w * y,
                0.0,
            ],
            [
                2.0 * x * y - 2.0 * w * z,
                w.powi(2) - x.powi(2) + y.powi(2) - z.powi(2),
                2.0 * y * z + 2.0 * w * x,
                0.0,
            ],
            [
                2.0 * x * z + 2.0 * w * y,
                2.0 * y * z - 2.0 * w * x,
                w.powi(2) - x.powi(2) - y.powi(2) + z.powi(2),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}
