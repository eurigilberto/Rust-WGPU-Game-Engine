use std::{cell::RefCell, rc::Rc};

use super::slot_index_types::SlotIndex;
///A ***Bucket*** is a collection of free slots
pub struct FreeBucket {
    start_index: usize,
    end_index: usize,
    pub next_bucket: Option<Rc<RefCell<FreeBucket>>>,
}

impl FreeBucket {
    fn new_single_slot(slot_index: SlotIndex) -> Self {
        Self {
            start_index: slot_index,
            end_index: slot_index + 1,
            next_bucket: None,
        }
    }
    fn new_multiple_slot(slot_index: SlotIndex, capacity: usize) -> Self {
        Self {
            start_index: slot_index,
            end_index: slot_index + capacity,
            next_bucket: None,
        }
    }
    fn size(&self) -> usize {
        self.end_index - self.start_index
    }
    fn take_single_slot(&mut self) -> SlotIndex {
        let slot = self.start_index;
        self.start_index += 1;
        slot
    }
    fn get_next_bucket(&self) -> Option<Rc<RefCell<FreeBucket>>> {
        match &self.next_bucket {
            Some(next_node) => Some(Rc::clone(next_node)),
            None => None,
        }
    }
}

/*impl Drop for FreeBucket {
    fn drop(&mut self) {
        println!("{} - {} Space dropped", self.start_index, self.end_index);
    }
}*/

pub struct FreeList {
    head: Option<Rc<RefCell<FreeBucket>>>,
}

impl FreeList {
    pub fn new(capacity: usize) -> Self {
        let free_space = FreeBucket::new_multiple_slot(0, capacity);

        FreeList {
            head: Some(Rc::new(RefCell::new(free_space))),
        }
    }

    pub fn get_free_slot(&mut self) -> Option<SlotIndex> {
        match &self.head {
            Some(head) => {
                let size = RefCell::borrow(head).size();

                if size > 1 {
                    //The current head can be shrunk to acomodate the new element
                    Some(head.borrow_mut().take_single_slot())
                } else if size == 1 {
                    //The current head cannot be shrunk
                    let slot_index = RefCell::borrow(head).start_index;
                    let next_node = RefCell::borrow(head).get_next_bucket();
                    //The current head is going to be replaced with the next node
                    if let Some(next_node) = next_node {
                        head.borrow_mut().next_bucket = None;
                        self.head.replace(next_node);
                    } else {
                        self.head = None;
                    }
                    Some(slot_index)
                } else {
                    None
                }
            }
            //There are no free slots
            None => None,
        }
    }

    pub fn merge_buckets(
        growing_bucket: Rc<RefCell<FreeBucket>>,
        disapearing_bucket: Rc<RefCell<FreeBucket>>,
    ) {
        let disapearing_bucket_borrow = RefCell::borrow(&disapearing_bucket);
        let mut growing_bucket_mut = RefCell::borrow_mut(&growing_bucket);
        growing_bucket_mut.end_index = disapearing_bucket_borrow.end_index;
        let next_bucket = disapearing_bucket_borrow.get_next_bucket();
        match next_bucket {
            Some(free_bucket) => {
                growing_bucket_mut.next_bucket.replace(free_bucket);
            }
            None => {
                growing_bucket_mut.next_bucket = None;
            }
        };
    }

    pub fn create_list_slice(&self) -> Vec<(usize, usize)> {
        let mut slice = Vec::<(usize, usize)>::new();
        if let Some(head) = &self.head {
            let mut current_bucket = Rc::clone(head);
            //println!("Create free list?");
            loop {
                let (start, end) = (
                    RefCell::borrow(&current_bucket).start_index,
                    RefCell::borrow(&current_bucket).end_index,
                );
                slice.push((start, end));

                let next_bucket = RefCell::borrow(&current_bucket).get_next_bucket();
                match next_bucket {
                    Some(next_bucket) => {
                        current_bucket = Rc::clone(&next_bucket);
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        slice
    }

    pub fn len(&self) -> usize {
        let slice = self.create_list_slice();
        slice.len()
    }

    pub fn get_tail(&self) -> Option<Rc<RefCell<FreeBucket>>>{
        match &self.head {
            Some(head) => {
                let mut current_tail = Rc::clone(head);
                //println!("Get Tail?");
                loop {
                    let next_bucket = RefCell::borrow(&current_tail).get_next_bucket();
                    match next_bucket {
                        Some(next_bucket) => {
                            current_tail = next_bucket;
                        },
                        None => {break;},
                    }
                }

                Some(Rc::clone(&current_tail))
            },
            None => None, // If there is no head, then the  
        }
    }

    /// Adds a bucket to the tail or grows the current tail if possible 
    pub fn add_free_bucket_to_tail(&mut self, index: SlotIndex, size: usize){
        let new_bucket = FreeBucket::new_multiple_slot(index, size);
        let new_bucket = Rc::new(RefCell::new(new_bucket));
        match self.get_tail() {
            Some(tail) => {
                let tail_end_index = RefCell::borrow(&tail).end_index;
                if tail_end_index == index{
                    //Merge new bucket with existing tail, as it would be next to it in memory
                    FreeList::merge_buckets(tail, new_bucket);
                } else {
                    //New bucket is not connected to the existing tail, so it is going to become the new tail
                    RefCell::borrow_mut(&tail).next_bucket.replace(new_bucket);
                }
            },
            None => {
                // There are no buckets so this bucket is going to be added as the head
                self.head.replace(new_bucket);
            },
        }
    }

    pub fn add_free_slot(&mut self, slot_index: SlotIndex) {
        match &self.head {
            Some(head) => {
                let head_start_index = RefCell::borrow(head).start_index;
                println!(
                    "Head start index {} || Slot index {}",
                    head_start_index, slot_index
                );

                if slot_index < head_start_index {
                    //The current slot is before the head
                    if head_start_index - 1 == slot_index {
                        //If the new slot is right before the current head, then expand backwards
                        RefCell::borrow_mut(head).start_index -= 1;
                    } else {
                        //It is farther back than a single slot away from the head
                        //Create a new bucket and make it the new head

                        let current_head = Rc::clone(head);
                        let mut new_bucket = FreeBucket::new_single_slot(slot_index);
                        new_bucket.next_bucket = Some(current_head);
                        self.head.replace(Rc::new(RefCell::new(new_bucket)));

                        #[cfg(test)]
                        {
                            println!("Changed head");
                        }
                    }
                } else {
                    let mut current_bucket = Rc::clone(head);
                    //println!("Free slot?");
                    loop {
                        let range_start = RefCell::borrow(&current_bucket).end_index;
                        let next_bucket = RefCell::borrow(&current_bucket).get_next_bucket();
                        match &next_bucket {
                            Some(next_bucket) => {
                                //there is a next bucket
                                let range_end = RefCell::borrow(next_bucket).start_index;

                                #[cfg(test)]
                                {
                                    println!("Range start {} || end {}", range_start, range_end);
                                }

                                if slot_index >= range_start && slot_index < range_end {
                                    //The new free slot is in between the current bucket and the next bucket

                                    if (range_end - range_start) == 1 {
                                        //the free slot would merge the current and next bucket
                                        FreeList::merge_buckets(
                                            Rc::clone(&current_bucket),
                                            Rc::clone(next_bucket),
                                        );
                                        #[cfg(test)]
                                        {
                                            println!("Merged buckets");
                                        }
                                        break;
                                    } else if range_start == slot_index {
                                        //the free slot needs to be attached to the current bucket
                                        RefCell::borrow_mut(&current_bucket).end_index += 1;
                                        #[cfg(test)]
                                        {
                                            println!("Expand current bucket");
                                        }
                                    } else if range_end == slot_index + 1 {
                                        //the free slot grows the next bucket backwards
                                        RefCell::borrow_mut(&next_bucket).start_index -= 1;
                                        #[cfg(test)]
                                        {
                                            println!("Expand next bucket");
                                        }
                                    } else {
                                        //A new bucket needs to be created because the free slot is in between the current and next buckets without touching either
                                        //the current bucket is going to point to the new bucket, and the new bucket is going to point to the next bucket
                                        let mut new_bucket =
                                            FreeBucket::new_single_slot(slot_index);
                                        new_bucket.next_bucket = Some(Rc::clone(next_bucket));
                                        RefCell::borrow_mut(&current_bucket)
                                            .next_bucket
                                            .replace(Rc::new(RefCell::new(new_bucket)));

                                        #[cfg(test)]
                                        {
                                            println!("Create bucket in between");
                                        }
                                    }
                                    break; //Free slot added into the list
                                } else {
                                    //The slot is not between the current and next bucket, continue searching
                                    current_bucket = Rc::clone(next_bucket);

                                    #[cfg(test)]
                                    {
                                        println!("Continue -----------------");
                                    }
                                    continue; //Continue searching
                                }
                            }
                            None => {
                                //There is no next bucket
                                let mut current_bucket = RefCell::borrow_mut(&current_bucket);
                                if current_bucket.end_index == slot_index {
                                    //The new free slot is right in the end of the current bucket, so the current bucket just needs to grow
                                    current_bucket.end_index += 1;
                                } else {
                                    //The new free slot is after
                                    current_bucket.next_bucket = Some(Rc::new(RefCell::new(
                                        FreeBucket::new_single_slot(slot_index),
                                    )));

                                    #[cfg(test)]
                                    {
                                        println!("Create bucket after?");
                                    }
                                }
                                break; //Free slot added into the list
                            }
                        }
                    }
                }
            }
            None => {
                //There were no free slots before, so this is the new head
                self.head
                    .replace(Rc::new(RefCell::new(FreeBucket::new_single_slot(
                        slot_index,
                    ))));
                //Free slot added into the list
            }
        }
    }
}
