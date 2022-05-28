#[cfg(test)]
mod tests {
    use crate::slotmap::slotmap::{PushResult, SlotKey, Slotmap};

    #[test]
    fn single_value_can_be_pushed_into_empty_slotmap() {
        let mut u32_slotmap = Slotmap::<u32>::new_with_capacity(100);
        let new_value_key = u32_slotmap.push(20);
        match new_value_key {
            PushResult::Result(val) => {
                assert_eq!(
                    u32_slotmap.free_list_len(),
                    1,
                    "Free List does not have the correct lenght"
                );
            }
            _ => {
                panic!("Could not push a value")
            }
        };
    }
    #[test]
    fn multiple_values_can_be_pushed_into_empty_slotmap() {
        let mut u32_slotmap = Slotmap::<u32>::new_with_capacity(100);
        for i in 0..25 {
            match u32_slotmap.push(i) {
                PushResult::Result(_) => {}
                PushResult::CapacityOverflow | PushResult::NoFreeSlotsAvailable => {
                    panic!("Could not push a value")
                }
            }
        }
        assert_eq!(u32_slotmap.capacity(), 100);
        assert_eq!(u32_slotmap.len(), 25, "Slot map should have 25");
        assert_eq!(
            u32_slotmap.free_list_len(),
            1,
            "Free List does not have the correct lenght"
        );
    }
    #[test]
    fn removing_a_value_from_partially_filled_slotmap_frees_a_slot() {
        let mut u32_slotmap = Slotmap::<u32>::new_with_capacity(100);
        let mut slot_keys = Vec::<SlotKey>::with_capacity(25);
        for i in 0..24 {
            if let PushResult::Result(key) = u32_slotmap.push(i) {
                slot_keys.push(key);
            } else {
                panic!("Could not push value")
            }
        }
        match u32_slotmap.remove(slot_keys[0]) {
            Some(value) => {
                assert_eq!(value, 0);
                assert_eq!(
                    u32_slotmap.free_list_len(),
                    2,
                    "Free List does not have the correct lenght"
                );
                match u32_slotmap.get_value(slot_keys[0]) {
                    Some(_) => {
                        panic!("Removing a value should make the key invalid")
                    }
                    None => {}
                }
            }
            None => {
                panic!("No value was returned on remove for a known valid key")
            }
        };
    }
    #[test]
	fn removing_multiple_values_from_partially_filled_slotmap__frees_multiple_slots() {
        let mut u32_slotmap = Slotmap::<u32>::new_with_capacity(100);
        let mut slot_keys = Vec::<SlotKey>::with_capacity(25);
        for i in 0..24 {
            if let PushResult::Result(key) = u32_slotmap.push(i) {
                slot_keys.push(key);
            } else {
                panic!("Could not push value")
            }
        }

        let check_value = |expected_value: u32, slot_result: Option<u32>| match slot_result {
            Some(value) => {
                assert_eq!(
                    value, expected_value,
                    "Resulting value -{}- is not the expected one -{}-",
                    value, expected_value
                );
				println!("Removed element {}", value);
            }
            None => {
                panic!("Expected a value to be returned")
            }
        };

		check_value(0, u32_slotmap.remove(slot_keys[0]));
		check_value(1, u32_slotmap.remove(slot_keys[1]));
        check_value(2, u32_slotmap.remove(slot_keys[2]));
		check_value(4, u32_slotmap.remove(slot_keys[4]));
    }
	#[test]
	fn removing_multiple_values_randomly_from_partially_filled_slotmap_frees_multiple_slots(){
		let mut u32_slotmap = Slotmap::<u32>::new_with_capacity(100);
        let mut slot_keys = Vec::<(u32, SlotKey)>::with_capacity(25);
        for i in 0..24 {
            if let PushResult::Result(key) = u32_slotmap.push(i) {
                slot_keys.push((i, key));
            } else {
                panic!("Could not push value")
            }
        }
		
        let check_value = |expected_value: u32, slot_result: Option<u32>| match slot_result {
			Some(value) => {
                assert_eq!(
                    value, expected_value,
                    "Resulting value -{}- is not the expected one -{}-",
                    value, expected_value
                );
            }
            None => {
                panic!("Expected a value to be returned");
            }
        };

		while slot_keys.len() > 10 {
			println!("--------------------------");
			let rand_index = rand::random::<usize>() % slot_keys.len();
			let key = slot_keys.remove(rand_index);
			println!("Trying to remove slotkey {:?}", key.1);
			
			check_value(key.0, u32_slotmap.remove(key.1));
		}

		println!("Free list slice {:?}", u32_slotmap.free_list_slice());
	}
    #[test]
    fn removing_multiple_values_can_make_free_buckets_to_merge_on_a_partially_filled_slotmap(){
        let mut u32_slotmap = Slotmap::<u32>::new_with_capacity(100);
        let mut slot_keys = Vec::<SlotKey>::with_capacity(25);
        for i in 0..24 {
            if let PushResult::Result(key) = u32_slotmap.push(i) {
                slot_keys.push(key);
            } else {
                panic!("Could not push value")
            }
        }

        let check_value = |expected_value: u32, slot_result: Option<u32>| match slot_result {
            Some(value) => {
                assert_eq!(
                    value, expected_value,
                    "Resulting value -{}- is not the expected one -{}-",
                    value, expected_value
                );
				println!("Removed element {}", value);
            }
            None => {
                panic!("Expected a value to be returned")
            }
        };

		check_value(0, u32_slotmap.remove(slot_keys[0]));
        check_value(2, u32_slotmap.remove(slot_keys[2]));
        check_value(1, u32_slotmap.remove(slot_keys[1]));

		check_value(4, u32_slotmap.remove(slot_keys[4]));

        let slice = u32_slotmap.free_list_slice();
        assert_eq!(slice[0], (0, 3));
    }
    #[test]
    fn an_iterator_can_be_created_from_a_partially_filled_slotmap(){
        let mut u32_slotmap = Slotmap::<u32>::new_with_capacity(100);
        let mut slot_keys = Vec::<SlotKey>::with_capacity(25);
        for i in 0..24 {
            if let PushResult::Result(key) = u32_slotmap.push(i) {
                slot_keys.push(key);
            } else {
                panic!("Could not push value")
            }
        }
        let slice = u32_slotmap.free_list_slice();
        assert_eq!(slice[0], (24, 100));

        for (index, val) in u32_slotmap.get_iter().enumerate(){
            assert_eq!(index as u32, *val, "The stored value is not the correct one | expected {} - found {} |", index as u32, *val);
        }
    }
    #[test]
    fn a_mutable_iterator_can_be_created_from_a_partially_filled_slotmap(){
        let mut u32_slotmap = Slotmap::<u32>::new_with_capacity(100);
        let mut slot_keys = Vec::<SlotKey>::with_capacity(25);
        for i in 0..24 {
            if let PushResult::Result(key) = u32_slotmap.push(i) {
                slot_keys.push(key);
            } else {
                panic!("Could not push value")
            }
        }
        let slice = u32_slotmap.free_list_slice();
        assert_eq!(slice[0], (24, 100));

        for (index, val) in u32_slotmap.get_iter_mut().enumerate(){
            assert_eq!(index as u32, *val, "The stored value is not the correct one | expected {} - found {} |", index as u32, *val);
            *val += 2;
        }

        for (index, val) in u32_slotmap.get_iter().enumerate(){
            let expected_value = index as u32 + 2;
            assert_eq!(expected_value, *val, "The stored value is not the correct one | expected {} - found {} |", expected_value, *val);
        }
    }
}
