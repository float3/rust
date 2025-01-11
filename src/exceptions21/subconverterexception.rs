pub(crate) struct SubConverterException {
    music21exception: Music21Exception,
}

impl SubConverterException {
    pub(crate) fn new() -> SubConverterException {
        SubConverterException {
            music21exception: Music21Exception::new(),
        }
    }
    
}