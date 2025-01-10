pub(crate)  struct SlottedObjectMixin {
    __slots__: ,
}

impl SlottedObjectMixin {
    pub(crate)  fn new() -> SlottedObjectMixin {
        SlottedObjectMixin {
            __slots__: todo!(),
        }
    }
    
    pub(crate)  fn __getstate__(&self) {
        todo!()
    }
    pub(crate)  fn __setstate__(&self, state: ) {
        todo!()
    }
    pub(crate)  fn _getSlotsRecursive(&self) {
        todo!()
    }
}