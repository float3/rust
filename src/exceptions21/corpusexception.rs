pub struct CorpusException {
    music21exception: Music21Exception,
}

impl CorpusException {
    pub fn new() -> CorpusException {
        CorpusException {
            music21exception: Music21Exception::new(),
        }
    }
    
}