pub(crate)  struct MicrotoneException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MicrotoneException {
    pub(crate)  fn new() -> MicrotoneException {
        MicrotoneException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}