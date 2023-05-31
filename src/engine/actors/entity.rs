use crate::engine::primitives::{transformation::Transformation2D, vector::Vector2};
use crate::engine::traits::update_entity::UpdateEntity;

pub struct Entity2D {
    position: Vector2<u32>,
    rotation: f32,
    scale: f32,
    transformation: Transformation2D,
}

impl Entity2D {
    pub fn new(x: u32, y: u32, rotation: f32, scale: f32) -> Self {
        Self {
            position: Vector2 { x, y },
            rotation,
            scale,
            transformation: Transformation2D::new(rotation, scale),
        }
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn position(&self) -> Vector2<u32> {
        self.position
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.position.x = x;
        self.position.y = y;
    }
}

impl UpdateEntity for Entity2D {
    fn update(&mut self) {
        self.transformation.update(self.rotation, self.scale);
    }
}
