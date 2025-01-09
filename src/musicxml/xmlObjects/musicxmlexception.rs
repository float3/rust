pub struct MusicXMLException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MusicXMLException {
    pub fn new() -> MusicXMLException {
        MusicXMLException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
    pub fn new(&self, message: String) {
        todo!()
    }
    pub fn __str__(&self) {
        todo!()
    }
}