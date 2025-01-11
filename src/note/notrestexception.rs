pub(crate) struct NotRestException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl NotRestException {
    pub(crate) fn new() -> NotRestException {
        NotRestException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}