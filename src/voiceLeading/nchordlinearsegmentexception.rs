pub(crate) struct NChordLinearSegmentException {
    music21exception: Music21Exception,
}

impl NChordLinearSegmentException {
    pub(crate) fn new() -> NChordLinearSegmentException {
        NChordLinearSegmentException {
            music21exception: Music21Exception::new(),
        }
    }
    
}