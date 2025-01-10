pub(crate)  struct ArchiveManagerException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ArchiveManagerException {
    pub(crate)  fn new() -> ArchiveManagerException {
        ArchiveManagerException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}