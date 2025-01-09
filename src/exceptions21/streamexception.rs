pub struct StreamException {
    music21exception: Music21Exception,
}

impl StreamException {
    pub fn new() -> StreamException {
        StreamException {
            music21exception: Music21Exception::new(),
        }
    }
    
}