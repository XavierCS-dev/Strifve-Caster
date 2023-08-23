use core::num;
use std::f32::consts::PI;

use num_traits::{Float, NumCast};

use crate::engine::primitives::vector::Vector3;

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

    pub fn to_raw(&self) -> [[f32; 3]; 3] {
        let angle = (self.angle.to_f32().unwrap().to_radians() / 2.0).cos();
        let x = self.axis.x.to_f32().unwrap() * (angle / 2.0).sin();
        let y = self.axis.y.to_f32().unwrap() * (angle / 2.0).sin();
        let z = self.axis.z.to_f32().unwrap() * (angle / 2.0).sin();
        let axis = Vector3 { x, y, z };
        let two_s;
        if self.unit {
            two_s = 2.0;
        } else {
            two_s = 2.0 / axis.square_magnitude();
        }
        // https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation
        [
            [
                angle.powi(2) + x.powi(2) - y.powi(2) - z.powi(2),
                2.0 * x * y + 2.0 * angle * z,
                2.0 * x * z - 2.0 * angle * y,
            ],
            [
                2.0 * x * y - 2.0 * angle * z,
                angle.powi(2) - x.powi(2) + y.powi(2) - z.powi(2),
                2.0 * y * z + 2.0 * angle * x,
            ],
            [
                2.0 * x * z + 2.0 * angle * y,
                2.0 * y * z - 2.0 * angle * x,
                angle.powi(2) - x.powi(2) - y.powi(2) + z.powi(2),
            ],
        ]
    }
}
