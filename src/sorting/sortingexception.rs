use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct SortingException {
    music21exception: Music21Exception,
}

impl SortingException {
    pub(crate) fn new(error_message: String) -> SortingException {
        SortingException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for SortingException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SortingException: {}", self.music21exception)
    }
}

impl Error for SortingException {}

impl Music21ExceptionTrait for SortingException {}