/// This module is meant to define the slotmap and constraints a GUI container should have
use std::marker::PhantomData;

use std::ops::{Deref, DerefMut};

use glam::UVec2;

use crate::EngineDataType;
use crate::entity_component::{PublicDataCollection, EngineDataTypeKey};
use crate::slotmap::slotmap::Slotmap;

use super::event::UIEvent;

pub struct GUISlotmaps<K: From<EngineDataTypeKey>, V: From<EngineDataType>, E: GUIContainer<K,V>>(Slotmap<E>, PhantomData<K>, PhantomData<V>);

impl<K: From<EngineDataTypeKey>, V: From<EngineDataType>, E: GUIContainer<K,V>> GUISlotmaps<K,V, E> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Slotmap::<E>::new_with_capacity(capacity), PhantomData, PhantomData)
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
}

pub trait GUIContainer<K: From<EngineDataTypeKey>, V: From<EngineDataType>> {
    fn get_name(&self) -> &String;
    fn update(&self, event: &UIEvent, public_data: &mut PublicDataCollection<K,V>);
    fn allow_resize(&self) -> bool;
    fn get_size(&self) -> UVec2;
}