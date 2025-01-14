use crate::exceptions21::music21exception::{Music21Exception, Music21ExceptionTrait};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ABCHandlerException {
    music21exception: Music21Exception,
}

impl ABCHandlerException {
    pub(crate) fn new(error_message: String) -> ABCHandlerException {
        ABCHandlerException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ABCHandlerException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ABCHandlerException: {}", self.music21exception)
    }
}

impl Error for ABCHandlerException {}

impl Music21ExceptionTrait for ABCHandlerException {}
