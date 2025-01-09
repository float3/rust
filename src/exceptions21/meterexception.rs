pub struct MeterException {
    music21exception: Music21Exception,
}

impl MeterException {
    pub fn new() -> MeterException {
        MeterException {
            music21exception: Music21Exception::new(),
        }
    }
    
}