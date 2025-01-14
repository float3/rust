pub(crate) struct SortingException {
    music21exception: Music21Exception,
}

impl SortingException {
    pub(crate) fn new() -> SortingException {
        SortingException {
            music21exception: Music21Exception::new(),
        }
    }
    
}