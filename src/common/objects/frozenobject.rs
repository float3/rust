pub(crate) struct FrozenObject {
    equalslottedobjectmixin: EqualSlottedObjectMixin,

}

impl FrozenObject {
    pub(crate) fn new() -> FrozenObject {
        FrozenObject {
            equalslottedobjectmixin: EqualSlottedObjectMixin::new(),

        }
    }
    
    pub(crate) fn _check_init(&self, key: ) -> bool {
        todo!()
    }
    pub(crate) fn __setattr__(&self, key: String, value: ) {
        todo!()
    }
    pub(crate) fn __delattr__(&self, key: String) {
        todo!()
    }
    pub(crate) fn __setitem__(&self, key: ) {
        todo!()
    }
    pub(crate) fn __delitem__(&self, key: ) {
        todo!()
    }
    pub(crate) fn __hash__(&self) -> IntegerType {
        todo!()
    }
}