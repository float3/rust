pub(crate) struct LyricException {
    music21exception: Music21Exception,
}

impl LyricException {
    pub(crate) fn new() -> LyricException {
        LyricException {
            music21exception: Music21Exception::new(),
        }
    }
    
}