pub struct ElementException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ElementException {
    pub fn new() -> ElementException {
        ElementException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}