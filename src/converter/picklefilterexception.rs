pub(crate) struct PickleFilterException {
    music21exception: Music21Exception,
}

impl PickleFilterException {
    pub(crate) fn new() -> PickleFilterException {
        PickleFilterException {
            music21exception: Music21Exception::new(),
        }
    }
    
}