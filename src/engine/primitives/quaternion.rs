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
        let angle = (self.angle.to_f32().unwrap() * PI) / 180.0;
        let x = self.axis.x.to_f32().unwrap();
        let y = self.axis.y.to_f32().unwrap();
        let z = self.axis.z.to_f32().unwrap();
        let mut normal = Vector3 { x, y, z };
        normal.normalise();
        let x = normal.x;
        let y = normal.y;
        let z = normal.z;
        let mut two_s = 0.0;
        if self.unit {
            two_s = 2.0;
        } else {
            two_s = 2.0 / normal.square_magnitude();
        }
        // https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation
        [
            [
                1.0 - two_s * (y.powi(2) + z.powi(2)),
                two_s * (x * y + z * angle),
                two_s * (x * z - y * angle),
            ],
            [
                two_s * (x * z - z * angle),
                1.0 - two_s * (x.powi(2) + z.powi(2)),
                two_s * (y * z + x * angle),
            ],
            [
                two_s * (x * z + y * angle),
                two_s * (y * z - x * angle),
                1.0 - two_s * (x.powi(2) + y.powi(2)),
            ],
        ]
    }
}
