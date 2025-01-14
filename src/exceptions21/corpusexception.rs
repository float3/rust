use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct CorpusException {
    music21exception: Music21Exception,
}

impl CorpusException {
    pub(crate) fn new(error_message: String) -> CorpusException {
        CorpusException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for CorpusException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "CorpusException: {}", self.music21exception)
    }
}

impl Error for CorpusException {}

impl Music21ExceptionTrait for CorpusException {}