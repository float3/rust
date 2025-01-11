pub(crate) struct EqualSlottedObjectMixin {
    slottedobjectmixin: SlottedObjectMixin,

}

impl EqualSlottedObjectMixin {
    pub(crate) fn new() -> EqualSlottedObjectMixin {
        EqualSlottedObjectMixin {
            slottedobjectmixin: SlottedObjectMixin::new(),

        }
    }
    
    pub(crate) fn __eq__(&self, other: ) {
        todo!()
    }
    pub(crate) fn __ne__(&self, other: ) {
        todo!()
    }
}