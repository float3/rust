pub(crate) struct ElementException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ElementException {
    pub(crate) fn new() -> ElementException {
        ElementException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}