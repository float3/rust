pub(crate) struct MusicXMLException {
    music21exception: Music21Exception,
}

impl MusicXMLException {
    pub(crate) fn new() -> MusicXMLException {
        MusicXMLException {
            music21exception: Music21Exception::new(),
        }
    }
    
    pub(crate) fn new(message: String) {
        todo!()
    }
    pub(crate) fn __str__(&self) {
        todo!()
    }
}