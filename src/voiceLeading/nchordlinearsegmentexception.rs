pub struct NChordLinearSegmentException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl NChordLinearSegmentException {
    pub fn new() -> NChordLinearSegmentException {
        NChordLinearSegmentException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}