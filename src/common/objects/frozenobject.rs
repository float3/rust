use crate::defaults::StringType;
use std::hash::{Hash, Hasher};

use super::equalslottedobjectmixin::EqualSlottedObjectMixin;

pub(crate) struct FrozenObject {
    equalslottedobjectmixin: EqualSlottedObjectMixin,
}

impl FrozenObject {
    pub(crate) fn new() -> FrozenObject {
        FrozenObject {
            equalslottedobjectmixin: EqualSlottedObjectMixin::new(),
        }
    }

    pub(crate) fn _check_init(&self, key: Option<&str>) -> bool {
        if let Some("__class__") = key {
            return true;
        }
        if !self.equalslottedobjectmixin.frozen {
            return true;
        }

        // Note: Rust does not have a direct equivalent of Python's inspect.stack()
        // You might need to use a different approach to achieve similar functionality
        // For now, we will return false to indicate immutability

        false
    }
    pub(crate) fn __setattr__(&self, key: String, value: StringType) {
        todo!()
    }
    pub(crate) fn __delattr__(&self, key: String) {
        todo!()
    }
    pub(crate) fn __setitem__(&self, key: String, value: StringType) {
        if self.equalslottedobjectmixin.has_super_setitem() {
            self._check_init(Some(&key));
            self.equalslottedobjectmixin.super_setitem(key, value);
        } else {
            panic!(
                "{} object is not subscriptable",
                std::any::type_name::<Self>()
            );
        }
    }

    pub(crate) fn __delitem__(&self, key: String) {
        if self.equalslottedobjectmixin.has_super_delitem() {
            self._check_init(Some(&key));
            self.equalslottedobjectmixin.super_delitem(key);
        } else {
            panic!(
                "{} object is not subscriptable",
                std::any::type_name::<Self>()
            );
        }
    }

    fn _get_slots_recursive(&self) -> Vec<String> {
        self.equalslottedobjectmixin._get_slots_recursive()
    }
}

impl Hash for FrozenObject {
    fn hash<H: Hasher>(&self, state: &mut H) {}
}
