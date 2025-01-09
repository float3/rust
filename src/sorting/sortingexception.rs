pub struct SortingException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl SortingException {
    pub fn new() -> SortingException {
        SortingException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}