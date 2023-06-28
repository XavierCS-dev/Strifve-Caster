#[derive(Clone, Copy)]
pub struct Vector2<T>
where
    T: num_traits::Num,
    T: Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: num_traits::Num,
    T: Copy,
{
    pub fn to_raw(&self) -> [T; 2] {
        [self.x, self.y]
    }
}

pub struct Vector3<T>
where
    T: num_traits::Num,
    T: Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector3<T>
where
    T: num_traits::Num,
    T: Copy,
{
    pub fn to_raw(&self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}
