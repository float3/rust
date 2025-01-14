pub(crate) struct MeiValueError {
    music21exception: Music21Exception,
}

impl MeiValueError {
    pub(crate) fn new() -> MeiValueError {
        MeiValueError {
            music21exception: Music21Exception::new(),
        }
    }
    
}