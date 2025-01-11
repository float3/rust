pub(crate) struct TreeException {
    music21exception: Music21Exception,
}

impl TreeException {
    pub(crate) fn new() -> TreeException {
        TreeException {
            music21exception: Music21Exception::new(),
        }
    }
    
}