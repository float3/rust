pub(crate) struct ABCHandlerException {
    music21exception: Music21Exception,
}

impl ABCHandlerException {
    pub(crate) fn new() -> ABCHandlerException {
        ABCHandlerException {
            music21exception: Music21Exception::new(),
        }
    }
    
}