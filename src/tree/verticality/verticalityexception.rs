use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct VerticalityException {
    music21exception: Music21Exception,
}

impl VerticalityException {
    pub(crate) fn new(error_message: String) -> VerticalityException {
        VerticalityException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for VerticalityException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "VerticalityException: {}", self.music21exception)
    }
}

impl Error for VerticalityException {}

impl Music21ExceptionTrait for VerticalityException {}