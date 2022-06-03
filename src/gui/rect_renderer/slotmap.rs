/// This module is meant to define the slotmap and constraints a GUI container should have

use std::ops::{Deref, DerefMut};

use glam::UVec2;

use crate::slotmap::slotmap::Slotmap;

use super::event::UIEvent;

struct GUISlotmaps<E: GUIContainer>(Slotmap<E>);

impl<E: GUIContainer> GUISlotmaps<E> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Slotmap::<E>::new_with_capacity(capacity))
    }
}

impl<E: GUIContainer> Deref for GUISlotmaps<E> {
    type Target = Slotmap<E>;
    fn deref(&self) -> &Slotmap<E> {
        &self.0
    }
}

impl<E: GUIContainer> DerefMut for GUISlotmaps<E> {
    fn deref_mut(&mut self) -> &mut Slotmap<E> {
        &mut self.0
    }
}

trait GUIContainer {
    fn get_name(&self) -> &String;
    fn update(&self, event: &UIEvent);
    fn allow_resize(&self) -> bool;
    fn get_size(&self) -> UVec2;
}