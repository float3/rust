use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct MusicXMLExportException {
    music21exception: Music21Exception,
}

impl MusicXMLExportException {
    pub(crate) fn new(error_message: String) -> MusicXMLExportException {
        MusicXMLExportException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for MusicXMLExportException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MusicXMLExportException: {}", self.music21exception)
    }
}

impl Error for MusicXMLExportException {}

impl Music21ExceptionTrait for MusicXMLExportException {}