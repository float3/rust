use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct TextFormatException {
    music21exception: Music21Exception,
}

impl TextFormatException {
    pub(crate) fn new(error_message: String) -> TextFormatException {
        TextFormatException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for TextFormatException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TextFormatException: {}", self.music21exception)
    }
}

impl Error for TextFormatException {}

impl Music21ExceptionTrait for TextFormatException {}