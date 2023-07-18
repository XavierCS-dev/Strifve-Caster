use crate::engine::primitives::vector::Vector3;

pub struct Quarternion<T>
where
    T: num_traits::Num + std::marker::Copy,
{
    angle: T,
    axis: Vector3<T>,
}

impl<T> Quarternion<T>
where
    T: num_traits::Num + std::marker::Copy,
{
    pub fn new(axis: Vector3<T>, angle: T) -> Self {
        Self { angle, axis }
    }

    pub fn set_angle(&mut self, angle: T) {
        self.angle = angle;
    }

    pub fn set_axis(&mut self, axis: Vector3<T>) {
        self.axis = axis;
    }

    pub fn axis(&self) -> &Vector3<T> {
        &self.axis
    }

    pub fn angle(&self) -> T {
        self.angle
    }

    pub fn to_raw(&self) -> [[f32; 4]; 4] {
        todo!()
    }
}
