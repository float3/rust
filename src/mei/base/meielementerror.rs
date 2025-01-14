pub(crate) struct MeiElementError {
    music21exception: Music21Exception,
}

impl MeiElementError {
    pub(crate) fn new() -> MeiElementError {
        MeiElementError {
            music21exception: Music21Exception::new(),
        }
    }
    
}