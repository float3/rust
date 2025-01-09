pub struct ABCTokenException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ABCTokenException {
    pub fn new() -> ABCTokenException {
        ABCTokenException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}