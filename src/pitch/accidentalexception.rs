pub struct AccidentalException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl AccidentalException {
    pub fn new() -> AccidentalException {
        AccidentalException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}