pub struct AnalysisException {
    music21exception: Music21Exception,
}

impl AnalysisException {
    pub fn new() -> AnalysisException {
        AnalysisException {
            music21exception: Music21Exception::new(),
        }
    }
    
}