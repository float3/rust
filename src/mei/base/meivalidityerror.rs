pub(crate) struct MeiValidityError {
    music21exception: Music21Exception,
}

impl MeiValidityError {
    pub(crate) fn new() -> MeiValidityError {
        MeiValidityError {
            music21exception: Music21Exception::new(),
        }
    }
    
}