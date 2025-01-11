pub(crate) struct LyricException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl LyricException {
    pub(crate) fn new() -> LyricException {
        LyricException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}