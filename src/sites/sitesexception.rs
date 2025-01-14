pub(crate) struct SitesException {
    music21exception: Music21Exception,
}

impl SitesException {
    pub(crate) fn new() -> SitesException {
        SitesException {
            music21exception: Music21Exception::new(),
        }
    }
    
}