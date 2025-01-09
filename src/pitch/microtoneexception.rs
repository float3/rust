pub struct MicrotoneException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl MicrotoneException {
    pub fn new() -> MicrotoneException {
        MicrotoneException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}