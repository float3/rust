pub struct ChordTablesException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ChordTablesException {
    pub fn new() -> ChordTablesException {
        ChordTablesException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}