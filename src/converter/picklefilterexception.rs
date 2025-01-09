pub struct PickleFilterException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl PickleFilterException {
    pub fn new() -> PickleFilterException {
        PickleFilterException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}