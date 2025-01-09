pub struct TreeException {
    music21exception: Music21Exception,
}

impl TreeException {
    pub fn new() -> TreeException {
        TreeException {
            music21exception: Music21Exception::new(),
        }
    }
    
}