use std::collections::HashMap;

pub(crate) struct SlottedObjectMixin {}

impl SlottedObjectMixin {
    pub(crate) fn new() -> SlottedObjectMixin {
        SlottedObjectMixin {}
    }

    pub(crate) fn __getstate__(&self) -> HashMap<String, Option<String>> {
        let mut state = HashMap::new();

        if let Some(dict) = self.__dict__() {
            state.extend(dict.clone());
        }

        let slots = self._get_slots_recursive();
        for slot in slots {
            let s_value = self.get_slot_value(&slot);
            if let Some(ref_value) = s_value.as_ref() {
                println!(
                    "Warning: uncaught weakref found in {:?} - {}, will not be wrapped again",
                    self, slot
                );
            }
            state.insert(slot, s_value);
        }

        state
    }

    pub(crate) fn __setstate__(&mut self, state: HashMap<String, Option<String>>) {
        for (slot, value) in state {
            self.set_slot_value(&slot, value);
        }
    }

    pub(crate) fn _get_slots_recursive(&self) -> Vec<String> {
        let mut slots = Vec::new();
        let mut cls = Some(self.__class__());

        while let Some(current_cls) = cls {
            if let Some(current_slots) = current_cls.__slots__() {
                slots.extend(current_slots.iter().cloned());
            }
            cls = current_cls.superclass();
        }

        slots
    }
}
