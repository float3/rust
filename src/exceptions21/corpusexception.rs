pub(crate) struct CorpusException {
    music21exception: Music21Exception,
}

impl CorpusException {
    pub(crate) fn new() -> CorpusException {
        CorpusException {
            music21exception: Music21Exception::new(),
        }
    }
    
}