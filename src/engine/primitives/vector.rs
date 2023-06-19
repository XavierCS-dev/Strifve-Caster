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
