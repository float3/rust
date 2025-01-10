pub(crate)  struct EqualSlottedObjectMixin {
    slottedobjectmixin: SlottedObjectMixin,
    __slots__: ,
}

impl EqualSlottedObjectMixin {
    pub(crate)  fn new() -> EqualSlottedObjectMixin {
        EqualSlottedObjectMixin {
            slottedobjectmixin: SlottedObjectMixin::new(),
            __slots__: todo!(),
        }
    }
    
    pub(crate)  fn __eq__(&self, other: ) {
        todo!()
    }
    pub(crate)  fn __ne__(&self, other: ) {
        todo!()
    }
}