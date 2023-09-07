use num_traits::{real::Real, Float, NumCast};

#[derive(Clone, Copy)]
pub struct Vector2<T>
where
    T: num_traits::Num + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: num_traits::Num + Copy,
{
    pub fn to_raw(&self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<T> Vector2<T>
where
    T: num_traits::Num + Copy + NumCast + Float,
{
    pub fn normalise(&mut self) {
        let magnitude = self.magnitude();
        self.x = self.x / magnitude;
        self.y = self.y / magnitude;
    }

    pub fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn square_magnitude(&self) -> T {
        self.x.powi(2) + self.y.powi(2)
    }
}

#[derive(Copy, Clone)]
pub struct Vector3<T>
where
    T: num_traits::Num + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: num_traits::Num + Copy,
{
    pub fn to_raw(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T> Vector3<T>
where
    T: num_traits::Num + Copy + NumCast + Float,
{
    pub fn normalise(&mut self) {
        let magnitude = self.magnitude();
        self.x = self.x / magnitude;
        self.y = self.y / magnitude;
        self.z = self.z / magnitude;
    }

    pub fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn square_magnitude(&self) -> T {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(&self, rhs: &Vector3<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(&self, rhs: &Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}
