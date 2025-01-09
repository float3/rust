pub struct ABCChord {
    abcnote: ABCNote,
}

impl ABCChord {
    pub fn new() -> ABCChord {
        ABCChord {
            abcnote: ABCNote::new(),
        }
    }
    
    pub fn new(&self, src: String) {
        todo!()
    }
    pub fn parse(&self, forceKeySignature: ) {
        todo!()
    }
}