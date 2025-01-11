pub(crate) struct DurationException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl DurationException {
    pub(crate) fn new() -> DurationException {
        DurationException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}