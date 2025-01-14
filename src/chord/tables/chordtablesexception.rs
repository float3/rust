pub(crate) struct ChordTablesException {
    music21exception: Music21Exception,
}

impl ChordTablesException {
    pub(crate) fn new() -> ChordTablesException {
        ChordTablesException {
            music21exception: Music21Exception::new(),
        }
    }
    
}