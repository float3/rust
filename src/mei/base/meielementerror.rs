pub(crate) struct MeiElementError {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MeiElementError {
    pub(crate) fn new() -> MeiElementError {
        MeiElementError {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}