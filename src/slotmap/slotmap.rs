use std::{slice::{Iter, IterMut}};

use super::{
    free_list::FreeList,
    slot_index_types::{Generation, SlotIndex, ValueIndex},
};

#[derive(Clone)]
pub struct Slot {
    index: ValueIndex,
    generation: Generation,
    taken: bool,
}
#[derive(Debug, Clone, Copy)]
pub struct SlotKey {
    index: SlotIndex,
    generation: Generation,
}

/// This structure is optimized in the following order iteration > random acess > pushing objects > removing objects
pub struct Slotmap<V> {
    values: Vec<V>,
    ///Array with the slot indexes for the slots that are pointing to a value, this array has the same order as the Values array
    values_slot: Vec<SlotIndex>,
    slots: Vec<Slot>,
    free_list: FreeList,
}

impl<V> Slotmap<V> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        let values = Vec::<V>::with_capacity(capacity);
        let values_slot = Vec::<SlotIndex>::with_capacity(capacity);
        let slots = vec![
            Slot {
                index: 0,
                generation: 0,
                taken: false
            };
            capacity
        ];

        let free_list = FreeList::new(capacity);

        Self {
            values,
            values_slot,
            slots,
            free_list,
        }
    }

    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

	pub fn free_list_len(&self)->usize{
		self.free_list.len()
	}

	pub fn free_list_slice(&self)->Vec<(usize, usize)>{
		self.free_list.create_list_slice()
	}

    pub fn get_iter(&self) -> Iter<'_, V> {
        self.values.iter()
    }

    pub fn get_iter_mut(&mut self) -> IterMut<'_, V> {
        self.values.iter_mut()
    }

    pub fn get_value(&self, key: SlotKey) -> Option<&V> {
        if self.is_valid(&key) {
            Some(&self.values[self.slots[key.index].index])
        } else {
            None
        }
    }

    pub fn get_value_mut(&mut self, key: SlotKey) -> Option<&mut V> {
        if self.is_valid(&key) {
            Some(&mut self.values[self.slots[key.index].index])
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[inline]
    pub fn is_valid(&self, key: &SlotKey) -> bool {
        self.slots[key.index].generation == key.generation
    }

    fn return_slot(&mut self, index: SlotIndex) {
        let slot = &mut self.slots[index];
        slot.generation += 1;
        slot.taken = false;
    }

    fn take_slot(&mut self, index: SlotIndex, v_index: ValueIndex) -> SlotKey {
        let slot = &mut self.slots[index];
        slot.taken = true;
        slot.index = v_index;
        SlotKey {
            index,
            generation: slot.generation,
        }
    }

    /// This function uses the `Vec::reserve_exact` internally to increase the available space
    /// If the `self.len() + aditional` is smaller than `self.capacity()`, then no space is allocated.\
    /// Increased capacity is equatl to `max( 0, self.len() + aditional - self.capacity() )`
    pub fn reserve_exact(&mut self, aditional: usize) -> Option<usize> {
        let current_capacity = self.capacity();
        
        self.values.reserve_exact(aditional);
        self.values_slot.reserve_exact(aditional);

        let new_capacity = self.capacity();

        let extra_capacity = new_capacity - current_capacity;
        if extra_capacity > 0 {
            self.free_list.add_free_bucket_to_tail(current_capacity, extra_capacity);
            return Some(extra_capacity)
        }
        None
    }

    pub fn remove(&mut self, key: SlotKey) -> Option<V> {
        if self.is_valid(&key) {
			//todo!("If the value that is going to be swaped is the last array element, then the swap is not perfromed and the element is just removed, the current code does not reflect that and panics in that case");
            let val_index = self.slots[key.index].index;
            
            let value = self.values.swap_remove(val_index);
            self.values_slot.swap_remove(val_index);

            //Make all the keys that pointed to that value invalid
            self.return_slot(key.index);

            //If slotmap is not empty, update the affected slot to make sure it points to the right index,
            //becuase its value was swaped, and now it is pointing to an invalid space in the vector
            //Check if 'val_index' is not equal to the current len, if it is, it means that the swap_remove
            //happened with the last element of the array, which means that no elements were moved
            if !self.values.is_empty() && val_index != self.values.len() {
                let slot_index = self.values_slot[val_index];
                assert_ne!(key.index, slot_index, "The slot index that should be updated and the previous slot index should not be the same");
                self.slots[slot_index].index = val_index;
            }

            //the slot on index {key.index} is free now
            self.free_list.add_free_slot(key.index);

            Some(value)
        } else {
            None
        }
    }

    pub fn push(&mut self, value: V) -> Option<SlotKey> {
        if self.values.capacity() == self.values.len() {
            None
        } else {
            self.values.push(value);
            //Get an available lot to put as a stable renference to the added element
            match self.free_list.get_free_slot() {
                Some(free_slot) => {
                    self.values_slot.push(free_slot);
                    let slot_key = self.take_slot(free_slot, self.values.len() - 1);

                    Some(slot_key)
                }
                None => None,
            }
        }
    }
}
