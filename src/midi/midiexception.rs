pub struct MidiException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MidiException {
    pub fn new() -> MidiException {
        MidiException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}