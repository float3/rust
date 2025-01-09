#[derive(PartialEq)]
pub struct ProtoM21Object {
    id: Option<i32>,
    // _classTupleCacheDict: ,
    // _classSetCacheDict: ,
    // __slots__: ,
}

impl ProtoM21Object {
    pub fn new(id: i32) -> ProtoM21Object {
        ProtoM21Object { id: Some(id) }
    }

    pub fn new_without_id() -> ProtoM21Object {
        ProtoM21Object { id: None }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn set_id(&mut self, new_id: i32) {
        self.id = Some(new_id);
    }

    // pub fn isClassOrSubclass(&self, classFilterList: ) -> bool {
    //     return !self.classSet().isdisjoint(classFilterList);
    // }
    // pub fn classes(&self) {
    //     todo!()
    // }
    // pub fn classSet(&self) {
    //     todo!()
    // }
    // pub fn __repr__(&self) -> String {
    //     todo!()
    // }
    // pub fn _reprInternal(&self) -> String {
    //     todo!()
    // }
}
