pub(crate)  struct AnalysisException {
    music21exception: Music21Exception,
}

impl AnalysisException {
    pub(crate)  fn new() -> AnalysisException {
        AnalysisException {
            music21exception: Music21Exception::new(),
        }
    }
    
}