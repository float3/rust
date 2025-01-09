pub struct ABCHandlerException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ABCHandlerException {
    pub fn new() -> ABCHandlerException {
        ABCHandlerException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}