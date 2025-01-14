use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct SubConverterException {
    music21exception: Music21Exception,
}

impl SubConverterException {
    pub(crate) fn new(error_message: String) -> SubConverterException {
        SubConverterException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for SubConverterException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SubConverterException: {}", self.music21exception)
    }
}

impl Error for SubConverterException {}

impl Music21ExceptionTrait for SubConverterException {}