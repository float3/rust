use crate::exceptions21::music21exception::{Music21Exception, Music21ExceptionTrait};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ABCTokenException {
    music21exception: Music21Exception,
}

impl ABCTokenException {
    pub(crate) fn new(error_message: String) -> ABCTokenException {
        ABCTokenException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ABCTokenException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ABCTokenException: {}", self.music21exception)
    }
}

impl Error for ABCTokenException {}

impl Music21ExceptionTrait for ABCTokenException {}
