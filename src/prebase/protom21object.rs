#[derive(PartialEq, Clone, Debug)]
pub(crate) struct ProtoM21Object {
    id: Option<IntegerType>,
    // _classTupleCacheDict: ,
    // _classSetCacheDict: ,
}

impl ProtoM21Object {
    pub(crate) fn new(id: Option<IntegerType>) -> ProtoM21Object {
        ProtoM21Object { id: id }
    }

    pub(crate) fn new_without_id() -> ProtoM21Object {
        ProtoM21Object { id: None }
    }

    pub(crate) fn id(&self) -> Option<IntegerType> {
        self.id
    }

    pub(crate) fn set_id(&mut self, new_id: IntegerType) {
        self.id = Some(new_id);
    }

    // pub(crate) fn isClassOrSubclass(&self, classFilterList: ) -> bool {
    //     return !self.classSet().isdisjoint(classFilterList);
    // }
    // pub(crate) fn classes(&self) {
    //     todo!()
    // }
    // pub(crate) fn classSet(&self) {
    //     todo!()
    // }
    // pub(crate) fn __repr__(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn _reprInternal(&self) -> String {
    //     todo!()
    // }
}
