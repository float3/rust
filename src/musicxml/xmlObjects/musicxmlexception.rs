pub(crate)  struct MusicXMLException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MusicXMLException {
    pub(crate)  fn new() -> MusicXMLException {
        MusicXMLException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
    pub(crate)  fn new(&self, message: String) {
        todo!()
    }
    pub(crate)  fn __str__(&self) {
        todo!()
    }
}