use std::collections::HashMap;
use amethyst::renderer::sprite::SpriteSheetHandle;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum SpriteSheetType {
    Background(String),
    Custom(String)
}

pub struct SpriteSheetMap {
    sheets : HashMap<SpriteSheetType, SpriteSheetHandle>
}

impl SpriteSheetMap {
    pub fn insert(&mut self, key : SpriteSheetType, value : SpriteSheetHandle) -> amethyst::Result<()> {
        if self.sheets.contains_key(&key) {
            return Err(amethyst::Error::from_string("key already in map"));
        }
        self.sheets.insert(key, value);
        Ok(())
    }

    pub fn replace(&mut self, key : SpriteSheetType, value : SpriteSheetHandle) {
        self.sheets.insert(key, value);
    }

    pub fn get(&self, key : &SpriteSheetType) -> Option<&SpriteSheetHandle> {
        self.sheets.get(key)
    }
}
