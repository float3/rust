pub(crate)  struct MeiAttributeError {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MeiAttributeError {
    pub(crate)  fn new() -> MeiAttributeError {
        MeiAttributeError {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}