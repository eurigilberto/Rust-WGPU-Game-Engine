/// This module is meant to define the slotmap and constraints a GUI container should have
use std::marker::PhantomData;

use std::ops::{Deref, DerefMut};

use glam::UVec2;

use crate::{Engine};

use super::event::UIEvent;

/*pub struct GUISlotmaps<K: From<EngineDataTypeKey>, V: From<EngineDataType>, E: GUIContainer<K,V>>(Slotmap<E>, PhantomData<K>, PhantomData<V>);

impl<K: From<EngineDataTypeKey>, V: From<EngineDataType>, E: GUIContainer<K,V>> GUISlotmaps<K,V, E> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Slotmap::<E>::new_with_capacity(capacity), PhantomData, PhantomData)
    }

    pub fn update_gui(&mut self, key: &SlotKey, event: &UIEvent, public_data: &mut PublicDataCollection<K,V>, engine: &Engine){
        let gui_container = self.0.get_value_mut(key).expect("Gui container not found").update(event, public_data, engine);
    }
}

impl<K: From<EngineDataTypeKey>, V: From<EngineDataType>, E: GUIContainer<K,V>> Deref for GUISlotmaps<K,V, E> {
    type Target = Slotmap<E>;
    fn deref(&self) -> &Slotmap<E> {
        &self.0
    }
}

impl<K: From<EngineDataTypeKey>, V: From<EngineDataType>, E: GUIContainer<K,V>> DerefMut for GUISlotmaps<K,V, E> {
    fn deref_mut(&mut self) -> &mut Slotmap<E> {
        &mut self.0
    }
}*/

/// P is the public data collection type
pub trait GUIContainer<P> {
    fn get_name(&self) -> &String;
    fn handle_event(&self, event: &mut UIEvent, public_data: &mut P, engine: &Engine);
    fn allow_resize(&self) -> bool;
    fn get_size(&self) -> UVec2;
}