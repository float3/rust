pub(crate) struct ABCTokenException {
    music21exception: Music21Exception,
}

impl ABCTokenException {
    pub(crate) fn new() -> ABCTokenException {
        ABCTokenException {
            music21exception: Music21Exception::new(),
        }
    }
    
}