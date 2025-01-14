use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct MusicXMLException {
    music21exception: Music21Exception,
}

impl MusicXMLException {
    pub(crate) fn new(error_message: String) -> MusicXMLException {
        MusicXMLException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for MusicXMLException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MusicXMLException: {}", self.music21exception)
    }
}

impl Error for MusicXMLException {}

impl Music21ExceptionTrait for MusicXMLException {}