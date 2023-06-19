use crate::engine::texture::Texture2D;
use crate::engine::texture::TEXTURE_IDS;
use crate::engine::traits::update_textures::UpdateTextures;
use anyhow::Result;
use std::collections::HashMap;

pub struct Texture2DMap(HashMap<u32, Texture2D>);

impl Texture2DMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn inner(&self) -> &HashMap<u32, Texture2D> {
        &self.0
    }
}

impl UpdateTextures for Texture2DMap {
    /// Adds a texture to the Texture2D HashMap
    fn add_texture(&mut self, texture: Texture2D) {
        if self.0.contains_key(&texture.id()) {
            // All instances of texture ids being incorrect (used or not found) should panic,
            // as this is not something to ever be used by the end user, and thus is an engine bug and
            // incorrect ids could cause some very hard to debug issues. A panic makes it clearer
            panic!("Texture ID ({}) already in HashMap", texture.id());
        }
        self.0.insert(texture.id(), texture);
    }

    /// Removes a texture from the Texture2D HashMap and frees the Texture ID for use.
    fn remove_texture(&mut self, id: u32) {
        if !self.0.contains_key(&id) {
            panic!("Texture ID ({}) not found", id);
        }
        unsafe {
            let mut tex_ids = TEXTURE_IDS.lock().unwrap();
            let pos = tex_ids.iter().position(|&x| x == id).unwrap();
            tex_ids.remove(pos);
        }
        self.0.remove(&id).expect(
            format!(
                "Texture ID ({}) was found in TEXTURE_IDS but not Texture2DMap, did you forget to add_texture()?",
                id
            )
            .as_str(),
        );
    }
}
