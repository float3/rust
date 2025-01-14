use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct SitesException {
    music21exception: Music21Exception,
}

impl SitesException {
    pub(crate) fn new(error_message: String) -> SitesException {
        SitesException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for SitesException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SitesException: {}", self.music21exception)
    }
}

impl Error for SitesException {}

impl Music21ExceptionTrait for SitesException {}