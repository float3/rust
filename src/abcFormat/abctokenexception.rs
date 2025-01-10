pub(crate)  struct ABCTokenException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ABCTokenException {
    pub(crate)  fn new() -> ABCTokenException {
        ABCTokenException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}