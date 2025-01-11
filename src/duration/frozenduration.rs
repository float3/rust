pub(crate) struct FrozenDuration {
    common::objects::frozenobject: common::objects::FrozenObject,

}

impl FrozenDuration {
    pub(crate) fn new() -> FrozenDuration {
        FrozenDuration {
            common::objects::frozenobject: common::objects::FrozenObject::new(),

        }
    }
    
    pub(crate) fn new(&self) {
        todo!()
    }
    pub(crate) fn __deepcopy__(&self, memo: ) {
        todo!()
    }
}