pub(crate)  struct ThreeNoteLinearSegmentException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ThreeNoteLinearSegmentException {
    pub(crate)  fn new() -> ThreeNoteLinearSegmentException {
        ThreeNoteLinearSegmentException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}