pub struct HumdrumException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl HumdrumException {
    pub fn new() -> HumdrumException {
        HumdrumException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}