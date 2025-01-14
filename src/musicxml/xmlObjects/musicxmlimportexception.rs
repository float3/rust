use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct MusicXMLImportException {
    music21exception: Music21Exception,
}

impl MusicXMLImportException {
    pub(crate) fn new(error_message: String) -> MusicXMLImportException {
        MusicXMLImportException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for MusicXMLImportException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MusicXMLImportException: {}", self.music21exception)
    }
}

impl Error for MusicXMLImportException {}

impl Music21ExceptionTrait for MusicXMLImportException {}