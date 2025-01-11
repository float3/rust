pub(crate) struct MeterException {
    music21exception: Music21Exception,
}

impl MeterException {
    pub(crate) fn new() -> MeterException {
        MeterException {
            music21exception: Music21Exception::new(),
        }
    }
    
}