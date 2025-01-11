pub(crate) struct NChordLinearSegmentException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl NChordLinearSegmentException {
    pub(crate) fn new() -> NChordLinearSegmentException {
        NChordLinearSegmentException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}