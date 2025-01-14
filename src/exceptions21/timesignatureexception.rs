use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct TimeSignatureException {
    music21exception: Music21Exception,
}

impl TimeSignatureException {
    pub(crate) fn new(error_message: String) -> TimeSignatureException {
        TimeSignatureException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for TimeSignatureException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TimeSignatureException: {}", self.music21exception)
    }
}

impl Error for TimeSignatureException {}

impl Music21ExceptionTrait for TimeSignatureException {}