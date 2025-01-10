pub(crate)  struct SitesException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl SitesException {
    pub(crate)  fn new() -> SitesException {
        SitesException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}