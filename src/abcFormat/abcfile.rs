pub struct ABCFile {
    prebase::protom21object: prebase::ProtoM21Object,
}

impl ABCFile {
    pub fn new() -> ABCFile {
        ABCFile {
            prebase::protom21object: prebase::ProtoM21Object::new(),
        }
    }
    
    pub fn new(&self, abcVersion: ) {
        todo!()
    }
    pub fn open(&self, filename: ) {
        todo!()
    }
    pub fn openFileLike(&self, fileLike: ) {
        todo!()
    }
    pub fn _reprInternal(&self) {
        todo!()
    }
    pub fn close(&self) {
        todo!()
    }
    pub fn read(&self, number: ) {
        todo!()
    }
    pub fn extractReferenceNumber(&self, strSrc: String, number: i32) -> String {
        todo!()
    }
    pub fn readstr(&self, strSrc: String, number: ) -> ABCHandler {
        todo!()
    }
}