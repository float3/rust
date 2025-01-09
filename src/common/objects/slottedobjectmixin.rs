pub struct SlottedObjectMixin {
    __slots__: ,
}

impl SlottedObjectMixin {
    pub fn new() -> SlottedObjectMixin {
        SlottedObjectMixin {
            __slots__: todo!(),
        }
    }
    
    pub fn __getstate__(&self) {
        todo!()
    }
    pub fn __setstate__(&self, state: ) {
        todo!()
    }
    pub fn _getSlotsRecursive(&self) {
        todo!()
    }
}