use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct GroupException {
    music21exception: Music21Exception,
}

impl GroupException {
    pub(crate) fn new(error_message: String) -> GroupException {
        GroupException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for GroupException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "GroupException: {}", self.music21exception)
    }
}

impl Error for GroupException {}

impl Music21ExceptionTrait for GroupException {}