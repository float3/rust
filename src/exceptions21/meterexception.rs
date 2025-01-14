use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct MeterException {
    music21exception: Music21Exception,
}

impl MeterException {
    pub(crate) fn new(error_message: String) -> MeterException {
        MeterException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for MeterException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MeterException: {}", self.music21exception)
    }
}

impl Error for MeterException {}

impl Music21ExceptionTrait for MeterException {}