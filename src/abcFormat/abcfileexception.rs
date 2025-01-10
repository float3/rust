pub(crate)  struct ABCFileException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ABCFileException {
    pub(crate)  fn new() -> ABCFileException {
        ABCFileException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}