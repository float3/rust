pub struct PitchException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl PitchException {
    pub fn new() -> PitchException {
        PitchException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}