pub(crate)  struct ABCNote {
    abctoken: ABCToken,
    __slots__: ,
}

impl ABCNote {
    pub(crate)  fn new() -> ABCNote {
        ABCNote {
            abctoken: ABCToken::new(),
            __slots__: todo!(),
        }
    }
    
    pub(crate)  fn new(&self, src: , carriedAccidental: String) {
        todo!()
    }
    pub(crate)  fn _splitChordSymbols(&self, strSrc: String) {
        todo!()
    }
    pub(crate)  fn getPitchName(&self, strSrc: String, forceKeySignature: ) {
        todo!()
    }
    pub(crate)  fn getQuarterLength(&self, strSrc: String, forceDefaultQuarterLength: ) -> f64 {
        todo!()
    }
    pub(crate)  fn parse(&self, forceDefaultQuarterLength: ) {
        todo!()
    }
}