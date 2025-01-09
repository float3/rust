pub struct SitesException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl SitesException {
    pub fn new() -> SitesException {
        SitesException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}