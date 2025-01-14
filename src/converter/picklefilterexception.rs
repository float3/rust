use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct PickleFilterException {
    music21exception: Music21Exception,
}

impl PickleFilterException {
    pub(crate) fn new(error_message: String) -> PickleFilterException {
        PickleFilterException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for PickleFilterException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "PickleFilterException: {}", self.music21exception)
    }
}

impl Error for PickleFilterException {}

impl Music21ExceptionTrait for PickleFilterException {}