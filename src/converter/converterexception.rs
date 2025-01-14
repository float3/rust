use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ConverterException {
    music21exception: Music21Exception,
}

impl ConverterException {
    pub(crate) fn new(error_message: String) -> ConverterException {
        ConverterException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ConverterException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ConverterException: {}", self.music21exception)
    }
}

impl Error for ConverterException {}

impl Music21ExceptionTrait for ConverterException {}