pub struct InstrumentException {
    music21exception: Music21Exception,
}

impl InstrumentException {
    pub fn new() -> InstrumentException {
        InstrumentException {
            music21exception: Music21Exception::new(),
        }
    }
    
}