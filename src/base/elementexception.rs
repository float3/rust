use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ElementException {
    music21exception: Music21Exception,
}

impl ElementException {
    pub(crate) fn new(error_message: String) -> ElementException {
        ElementException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ElementException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ElementException: {}", self.music21exception)
    }
}

impl Error for ElementException {}

impl Music21ExceptionTrait for ElementException {}