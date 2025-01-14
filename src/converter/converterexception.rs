pub(crate) struct ConverterException {
    music21exception: Music21Exception,
}

impl ConverterException {
    pub(crate) fn new() -> ConverterException {
        ConverterException {
            music21exception: Music21Exception::new(),
        }
    }
    
}