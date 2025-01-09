pub struct SubConverterException {
    music21exception: Music21Exception,
}

impl SubConverterException {
    pub fn new() -> SubConverterException {
        SubConverterException {
            music21exception: Music21Exception::new(),
        }
    }
    
}