pub(crate) struct MeiAttributeError {
    music21exception: Music21Exception,
}

impl MeiAttributeError {
    pub(crate) fn new() -> MeiAttributeError {
        MeiAttributeError {
            music21exception: Music21Exception::new(),
        }
    }
    
}