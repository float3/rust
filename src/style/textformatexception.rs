pub(crate) struct TextFormatException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl TextFormatException {
    pub(crate) fn new() -> TextFormatException {
        TextFormatException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}