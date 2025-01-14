pub(crate) struct MuseDataException {
    music21exception: Music21Exception,
}

impl MuseDataException {
    pub(crate) fn new() -> MuseDataException {
        MuseDataException {
            music21exception: Music21Exception::new(),
        }
    }
    
}