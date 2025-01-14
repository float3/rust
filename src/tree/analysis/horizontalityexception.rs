use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct HorizontalityException {
    music21exception: Music21Exception,
}

impl HorizontalityException {
    pub(crate) fn new(error_message: String) -> HorizontalityException {
        HorizontalityException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for HorizontalityException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "HorizontalityException: {}", self.music21exception)
    }
}

impl Error for HorizontalityException {}

impl Music21ExceptionTrait for HorizontalityException {}