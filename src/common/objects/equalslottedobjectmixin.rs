use super::slottedobjectmixin::SlottedObjectMixin;
use std::any::TypeId;

pub(crate) struct EqualSlottedObjectMixin {
    slottedobjectmixin: SlottedObjectMixin,
}

impl EqualSlottedObjectMixin {
    pub(crate) fn new() -> EqualSlottedObjectMixin {
        EqualSlottedObjectMixin {
            slottedobjectmixin: SlottedObjectMixin::new(),
        }
    }
}

impl PartialEq for EqualSlottedObjectMixin {
    fn eq(&self, other: &Self) -> bool {
        if TypeId::of::<Self>() != TypeId::of::<Self>() {
            return false;
        }
        for this_slot in self.slottedobjectmixin._get_slots_recursive() {
            if this_slot == "id" {
                continue;
            }
            if self.slottedobjectmixin.get_attr(this_slot)
                != other.slottedobjectmixin.get_attr(this_slot)
            {
                return false;
            }
        }
        true
    }
}

impl Eq for EqualSlottedObjectMixin {}
