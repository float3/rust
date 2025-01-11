pub(crate) struct ABCChord {
    abcnote: ABCNote,
}

impl ABCChord {
    pub(crate) fn new() -> ABCChord {
        ABCChord {
            abcnote: ABCNote::new(),
        }
    }
    
    pub(crate) fn new(&self, src: String) {
        todo!()
    }
    pub(crate) fn parse(&self, forceKeySignature: ) {
        todo!()
    }
}