pub struct LyricException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl LyricException {
    pub fn new() -> LyricException {
        LyricException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}