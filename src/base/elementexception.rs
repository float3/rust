pub(crate) struct ElementException {
    music21exception: Music21Exception,
}

impl ElementException {
    pub(crate) fn new() -> ElementException {
        ElementException {
            music21exception: Music21Exception::new(),
        }
    }
    
}