/// This module is meant to define the slotmap and constraints a GUI container should have
use std::marker::PhantomData;

use std::ops::{Deref, DerefMut};

use glam::UVec2;

use crate::slotmap::slotmap::Slotmap;

use super::event::UIEvent;

pub struct GUISlotmaps<T, E: GUIContainer<T>>(Slotmap<E>, PhantomData<T>);

impl<T, E: GUIContainer<T>> GUISlotmaps<T, E> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Slotmap::<E>::new_with_capacity(capacity), PhantomData)
    }
}

impl<T, E: GUIContainer<T>> Deref for GUISlotmaps<T, E> {
    type Target = Slotmap<E>;
    fn deref(&self) -> &Slotmap<E> {
        &self.0
    }
}

impl<T, E: GUIContainer<T>> DerefMut for GUISlotmaps<T, E> {
    fn deref_mut(&mut self) -> &mut Slotmap<E> {
        &mut self.0
    }
}

pub trait GUIContainer<T> {
    fn get_name(&self) -> &String;
    fn update(&self, event: &UIEvent, public_data: Slotmap<T>);
    fn allow_resize(&self) -> bool;
    fn get_size(&self) -> UVec2;
}