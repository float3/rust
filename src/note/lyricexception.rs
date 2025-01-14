use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct LyricException {
    music21exception: Music21Exception,
}

impl LyricException {
    pub(crate) fn new(error_message: String) -> LyricException {
        LyricException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for LyricException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LyricException: {}", self.music21exception)
    }
}

impl Error for LyricException {}

impl Music21ExceptionTrait for LyricException {}