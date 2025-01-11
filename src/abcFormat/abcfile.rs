pub(crate) struct ABCFile {
    prebase::protom21object: prebase::ProtoM21Object,
}

impl ABCFile {
    pub(crate) fn new() -> ABCFile {
        ABCFile {
            prebase::protom21object: prebase::ProtoM21Object::new(),
        }
    }
    
    pub(crate) fn new(&self, abcVersion: ) {
        todo!()
    }
    pub(crate) fn open(&self, filename: ) {
        todo!()
    }
    pub(crate) fn openFileLike(&self, fileLike: ) {
        todo!()
    }
    pub(crate) fn _reprInternal(&self) {
        todo!()
    }
    pub(crate) fn close(&self) {
        todo!()
    }
    pub(crate) fn read(&self, number: ) {
        todo!()
    }
    pub(crate) fn extractReferenceNumber(&self, strSrc: String, number: IntegerType) -> String {
        todo!()
    }
    pub(crate) fn readstr(&self, strSrc: String, number: ) -> ABCHandler {
        todo!()
    }
}