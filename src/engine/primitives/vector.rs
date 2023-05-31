#[derive(Clone, Copy)]
pub struct Vector2<T>
where
    T: num_traits::Num,
{
    pub x: T,
    pub y: T,
}
