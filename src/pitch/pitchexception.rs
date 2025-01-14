use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct PitchException {
    music21exception: Music21Exception,
}

impl PitchException {
    pub(crate) fn new(error_message: String) -> PitchException {
        PitchException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for PitchException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "PitchException: {}", self.music21exception)
    }
}

impl Error for PitchException {}

impl Music21ExceptionTrait for PitchException {}