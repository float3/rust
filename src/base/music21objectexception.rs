use crate::exceptions21::music21exception::{Music21Exception, Music21ExceptionTrait};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct Music21ObjectException {
    music21exception: Music21Exception,
}

impl Music21ObjectException {
    pub(crate) fn new(error_message: String) -> Music21ObjectException {
        Music21ObjectException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for Music21ObjectException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Music21ObjectException: {}", self.music21exception)
    }
}

impl Error for Music21ObjectException {}

impl Music21ExceptionTrait for Music21ObjectException {}
