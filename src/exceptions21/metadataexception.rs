pub(crate)  struct MetadataException {
    music21exception: Music21Exception,
}

impl MetadataException {
    pub(crate)  fn new() -> MetadataException {
        MetadataException {
            music21exception: Music21Exception::new(),
        }
    }
    
}