use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct NotRestException {
    music21exception: Music21Exception,
}

impl NotRestException {
    pub(crate) fn new(error_message: String) -> NotRestException {
        NotRestException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for NotRestException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "NotRestException: {}", self.music21exception)
    }
}

impl Error for NotRestException {}

impl Music21ExceptionTrait for NotRestException {}