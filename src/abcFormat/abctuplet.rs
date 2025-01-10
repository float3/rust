pub(crate)  struct ABCTuplet {
    abctoken: ABCToken,
    __slots__: ,
}

impl ABCTuplet {
    pub(crate)  fn new() -> ABCTuplet {
        ABCTuplet {
            abctoken: ABCToken::new(),
            __slots__: todo!(),
        }
    }
    
    pub(crate)  fn new(&self, src: String) {
        todo!()
    }
    pub(crate)  fn updateRatio(&self, timeSignatureObj: ) {
        todo!()
    }
    pub(crate)  fn updateNoteCount(&self) {
        todo!()
    }
}