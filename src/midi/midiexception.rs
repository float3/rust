pub(crate) struct MidiException {
    music21exception: Music21Exception,
}

impl MidiException {
    pub(crate) fn new() -> MidiException {
        MidiException {
            music21exception: Music21Exception::new(),
        }
    }
    
}