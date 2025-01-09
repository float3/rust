pub struct ABCNote {
    abctoken: ABCToken,
    __slots__: ,
}

impl ABCNote {
    pub fn new() -> ABCNote {
        ABCNote {
            abctoken: ABCToken::new(),
            __slots__: todo!(),
        }
    }
    
    pub fn new(&self, src: , carriedAccidental: String) {
        todo!()
    }
    pub fn _splitChordSymbols(&self, strSrc: String) {
        todo!()
    }
    pub fn getPitchName(&self, strSrc: String, forceKeySignature: ) {
        todo!()
    }
    pub fn getQuarterLength(&self, strSrc: String, forceDefaultQuarterLength: ) -> f64 {
        todo!()
    }
    pub fn parse(&self, forceDefaultQuarterLength: ) {
        todo!()
    }
}