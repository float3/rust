pub(crate) struct ConverterFileException {
    music21exception: Music21Exception,
}

impl ConverterFileException {
    pub(crate) fn new() -> ConverterFileException {
        ConverterFileException {
            music21exception: Music21Exception::new(),
        }
    }
    
}