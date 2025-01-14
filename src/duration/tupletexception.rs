use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct TupletException {
    music21exception: Music21Exception,
}

impl TupletException {
    pub(crate) fn new(error_message: String) -> TupletException {
        TupletException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for TupletException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TupletException: {}", self.music21exception)
    }
}

impl Error for TupletException {}

impl Music21ExceptionTrait for TupletException {}