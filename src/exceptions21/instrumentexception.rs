use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct InstrumentException {
    music21exception: Music21Exception,
}

impl InstrumentException {
    pub(crate) fn new(error_message: String) -> InstrumentException {
        InstrumentException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for InstrumentException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "InstrumentException: {}", self.music21exception)
    }
}

impl Error for InstrumentException {}

impl Music21ExceptionTrait for InstrumentException {}