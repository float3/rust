pub(crate) struct MeiValidityError {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MeiValidityError {
    pub(crate) fn new() -> MeiValidityError {
        MeiValidityError {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}