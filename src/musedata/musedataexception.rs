pub(crate)  struct MuseDataException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MuseDataException {
    pub(crate)  fn new() -> MuseDataException {
        MuseDataException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}