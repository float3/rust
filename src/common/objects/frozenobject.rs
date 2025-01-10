pub(crate)  struct FrozenObject {
    equalslottedobjectmixin: EqualSlottedObjectMixin,
    __slots__: ,
}

impl FrozenObject {
    pub(crate)  fn new() -> FrozenObject {
        FrozenObject {
            equalslottedobjectmixin: EqualSlottedObjectMixin::new(),
            __slots__: todo!(),
        }
    }
    
    pub(crate)  fn _check_init(&self, key: ) -> bool {
        todo!()
    }
    pub(crate)  fn __setattr__(&self, key: String, value: ) {
        todo!()
    }
    pub(crate)  fn __delattr__(&self, key: String) {
        todo!()
    }
    pub(crate)  fn __setitem__(&self, key: ) {
        todo!()
    }
    pub(crate)  fn __delitem__(&self, key: ) {
        todo!()
    }
    pub(crate)  fn __hash__(&self) -> i32 {
        todo!()
    }
}