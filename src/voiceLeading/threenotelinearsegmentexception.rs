pub(crate) struct ThreeNoteLinearSegmentException {
    music21exception: Music21Exception,
}

impl ThreeNoteLinearSegmentException {
    pub(crate) fn new() -> ThreeNoteLinearSegmentException {
        ThreeNoteLinearSegmentException {
            music21exception: Music21Exception::new(),
        }
    }
    
}