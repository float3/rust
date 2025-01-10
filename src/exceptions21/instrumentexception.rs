pub(crate)  struct InstrumentException {
    music21exception: Music21Exception,
}

impl InstrumentException {
    pub(crate)  fn new() -> InstrumentException {
        InstrumentException {
            music21exception: Music21Exception::new(),
        }
    }
    
}