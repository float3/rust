use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ArchiveManagerException {
    music21exception: Music21Exception,
}

impl ArchiveManagerException {
    pub(crate) fn new(error_message: String) -> ArchiveManagerException {
        ArchiveManagerException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ArchiveManagerException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ArchiveManagerException: {}", self.music21exception)
    }
}

impl Error for ArchiveManagerException {}

impl Music21ExceptionTrait for ArchiveManagerException {}