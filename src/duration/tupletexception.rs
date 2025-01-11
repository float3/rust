pub(crate) struct TupletException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl TupletException {
    pub(crate) fn new() -> TupletException {
        TupletException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}