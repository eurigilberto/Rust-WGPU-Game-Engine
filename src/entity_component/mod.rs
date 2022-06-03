use std::ops::{Deref, DerefMut};

use crate::{
    render_system::render_texture::RenderTexture,
    slotmap::slotmap::{SlotKey, Slotmap},
};

trait ExecutableComponent {
    fn update(&mut self);
}

//This adds a new type for slotmap that adds a trait constraint, so that more knowledge about the type can be used when writing
//the system code for function that are going to work the same for all types
struct ExecutableSlotmap<E: ExecutableComponent>(Slotmap<E>);

impl<E: ExecutableComponent> ExecutableSlotmap<E> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Slotmap::<E>::new_with_capacity(capacity))
    }
}

impl<E: ExecutableComponent> Deref for ExecutableSlotmap<E> {
    type Target = Slotmap<E>;
    fn deref(&self) -> &Slotmap<E> {
        &self.0
    }
}

impl<E: ExecutableComponent> DerefMut for ExecutableSlotmap<E> {
    fn deref_mut(&mut self) -> &mut Slotmap<E> {
        &mut self.0
    }
}

struct TestExecutable {
    pub value: f32,
}

impl ExecutableComponent for TestExecutable {
    fn update(&mut self) {
        println!("Testing thing {}", self.value);
    }
}

pub type RenderTextureSlotmap = Slotmap<RenderTexture>;

pub enum EngineDataSlotmapTypes {
    RenderTexture(RenderTextureSlotmap),
}

pub struct PublicDataSlotmap<T: Into<EngineDataSlotmapTypes> + From<EngineDataSlotmapTypes>> {
    pub data_containers: Vec<T>,
}

pub enum EngineSlotmapKeys {
    RenderTexture(SlotKey),
}

pub struct Entity<T: Into<EngineSlotmapKeys> + From<EngineSlotmapKeys>> {
    pub components: Vec<T>,
}
