pub(crate) struct TextFormatException {
    music21exception: Music21Exception,
}

impl TextFormatException {
    pub(crate) fn new() -> TextFormatException {
        TextFormatException {
            music21exception: Music21Exception::new(),
        }
    }
    
}