use crate::exceptions21::music21exception::{Music21Exception, Music21ExceptionTrait};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ABCFileException {
    music21exception: Music21Exception,
}

impl ABCFileException {
    pub(crate) fn new(error_message: String) -> ABCFileException {
        ABCFileException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ABCFileException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ABCFileException: {}", self.music21exception)
    }
}

impl Error for ABCFileException {}

impl Music21ExceptionTrait for ABCFileException {}
