pub struct MetadataException {
    music21exception: Music21Exception,
}

impl MetadataException {
    pub fn new() -> MetadataException {
        MetadataException {
            music21exception: Music21Exception::new(),
        }
    }
    
}