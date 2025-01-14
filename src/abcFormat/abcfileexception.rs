pub(crate) struct ABCFileException {
    music21exception: Music21Exception,
}

impl ABCFileException {
    pub(crate) fn new() -> ABCFileException {
        ABCFileException {
            music21exception: Music21Exception::new(),
        }
    }
    
}