use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ElementTreeException {
    music21exception: Music21Exception,
}

impl ElementTreeException {
    pub(crate) fn new(error_message: String) -> ElementTreeException {
        ElementTreeException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ElementTreeException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ElementTreeException: {}", self.music21exception)
    }
}

impl Error for ElementTreeException {}

impl Music21ExceptionTrait for ElementTreeException {}