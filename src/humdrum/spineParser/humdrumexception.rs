pub(crate) struct HumdrumException {
    music21exception: Music21Exception,
}

impl HumdrumException {
    pub(crate) fn new() -> HumdrumException {
        HumdrumException {
            music21exception: Music21Exception::new(),
        }
    }
    
}