use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ConverterFileException {
    music21exception: Music21Exception,
}

impl ConverterFileException {
    pub(crate) fn new(error_message: String) -> ConverterFileException {
        ConverterFileException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ConverterFileException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ConverterFileException: {}", self.music21exception)
    }
}

impl Error for ConverterFileException {}

impl Music21ExceptionTrait for ConverterFileException {}