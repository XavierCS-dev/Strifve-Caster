use crate::engine::texture::Texture2D;
use anyhow::Result;

pub trait UpdateTextures {
    /// Add a texture to the vec and queue
    fn add_texture(&mut self, texture: Texture2D);

    /// Remove a texture from the queue based on it's ID
    fn remove_texture(&mut self, id: u32);
}
