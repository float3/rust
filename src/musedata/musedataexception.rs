use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct MuseDataException {
    music21exception: Music21Exception,
}

impl MuseDataException {
    pub(crate) fn new(error_message: String) -> MuseDataException {
        MuseDataException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for MuseDataException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MuseDataException: {}", self.music21exception)
    }
}

impl Error for MuseDataException {}

impl Music21ExceptionTrait for MuseDataException {}