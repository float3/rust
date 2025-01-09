pub struct Music21ObjectException {
    music21exception: Music21Exception,
}

impl Music21ObjectException {
    pub fn new() -> Music21ObjectException {
        Music21ObjectException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}