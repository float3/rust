use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct MicrotoneException {
    music21exception: Music21Exception,
}

impl MicrotoneException {
    pub(crate) fn new(error_message: String) -> MicrotoneException {
        MicrotoneException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for MicrotoneException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MicrotoneException: {}", self.music21exception)
    }
}

impl Error for MicrotoneException {}

impl Music21ExceptionTrait for MicrotoneException {}