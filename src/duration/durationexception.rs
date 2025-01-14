use crate::exceptions21::music21exception::{Music21Exception, Music21ExceptionTrait};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct DurationException {
    music21exception: Music21Exception,
}

impl DurationException {
    pub(crate) fn new(error_message: String) -> DurationException {
        DurationException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for DurationException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.music21exception)
    }
}

impl Error for DurationException {}

impl Music21ExceptionTrait for DurationException {}
