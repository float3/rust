pub(crate) struct MidiException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MidiException {
    pub(crate) fn new() -> MidiException {
        MidiException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}