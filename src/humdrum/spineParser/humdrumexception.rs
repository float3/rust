use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct HumdrumException {
    music21exception: Music21Exception,
}

impl HumdrumException {
    pub(crate) fn new(error_message: String) -> HumdrumException {
        HumdrumException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for HumdrumException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "HumdrumException: {}", self.music21exception)
    }
}

impl Error for HumdrumException {}

impl Music21ExceptionTrait for HumdrumException {}