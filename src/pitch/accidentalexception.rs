use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct AccidentalException {
    music21exception: Music21Exception,
}

impl AccidentalException {
    pub(crate) fn new(error_message: String) -> AccidentalException {
        AccidentalException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for AccidentalException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "AccidentalException: {}", self.music21exception)
    }
}

impl Error for AccidentalException {}

impl Music21ExceptionTrait for AccidentalException {}