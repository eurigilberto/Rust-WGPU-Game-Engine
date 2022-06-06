use std::collections::HashMap;

use crate::{
    render_system::render_texture::RenderTexture,
    slotmap::slotmap::{SlotKey, Slotmap},
};

pub type RenderTextureSlotmap = Slotmap<RenderTexture>;

pub enum EngineDataType {
    RenderTexture(RenderTextureSlotmap),
}

#[derive(Hash, PartialEq, Eq)]
pub enum EngineDataTypeKey {
    RenderTexture,
}

pub struct EngineDataKey {
    pub map_key: EngineDataTypeKey,
    pub key: SlotKey,
}

pub struct PublicDataCollection<K: From<EngineDataTypeKey>, V: From<EngineDataType>> {
    pub collection: HashMap<K, V>,
}

impl<K: From<EngineDataTypeKey>, V: From<EngineDataType>> PublicDataCollection<K, V> {
    pub fn new() -> Self {
        Self {
            collection: HashMap::<K, V>::new(),
        }
    }
}

/*pub struct Entity<T: Into<EngineSlotmapKeys> + From<EngineSlotmapKeys>> {
    pub components: Vec<T>,
}*/
//???
