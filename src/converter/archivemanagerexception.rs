pub(crate) struct ArchiveManagerException {
    music21exception: Music21Exception,
}

impl ArchiveManagerException {
    pub(crate) fn new() -> ArchiveManagerException {
        ArchiveManagerException {
            music21exception: Music21Exception::new(),
        }
    }
    
}