pub struct MeiValueError {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MeiValueError {
    pub fn new() -> MeiValueError {
        MeiValueError {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}