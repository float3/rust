pub struct ChordException {
    exceptions21::music21exception:exceptions21::Music21Exception,
}

impl ChordException {
    pub fn new() -> ChordException {
        ChordException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}