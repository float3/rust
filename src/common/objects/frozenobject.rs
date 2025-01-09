pub struct FrozenObject {
    equalslottedobjectmixin: EqualSlottedObjectMixin,
    __slots__: ,
}

impl FrozenObject {
    pub fn new() -> FrozenObject {
        FrozenObject {
            equalslottedobjectmixin: EqualSlottedObjectMixin::new(),
            __slots__: todo!(),
        }
    }
    
    pub fn _check_init(&self, key: ) -> bool {
        todo!()
    }
    pub fn __setattr__(&self, key: String, value: ) {
        todo!()
    }
    pub fn __delattr__(&self, key: String) {
        todo!()
    }
    pub fn __setitem__(&self, key: ) {
        todo!()
    }
    pub fn __delitem__(&self, key: ) {
        todo!()
    }
    pub fn __hash__(&self) -> i32 {
        todo!()
    }
}