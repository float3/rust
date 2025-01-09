pub struct EqualSlottedObjectMixin {
    slottedobjectmixin: SlottedObjectMixin,
    __slots__: ,
}

impl EqualSlottedObjectMixin {
    pub fn new() -> EqualSlottedObjectMixin {
        EqualSlottedObjectMixin {
            slottedobjectmixin: SlottedObjectMixin::new(),
            __slots__: todo!(),
        }
    }
    
    pub fn __eq__(&self, other: ) {
        todo!()
    }
    pub fn __ne__(&self, other: ) {
        todo!()
    }
}