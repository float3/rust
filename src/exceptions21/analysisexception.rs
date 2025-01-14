use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct AnalysisException {
    music21exception: Music21Exception,
}

impl AnalysisException {
    pub(crate) fn new(error_message: String) -> AnalysisException {
        AnalysisException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for AnalysisException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "AnalysisException: {}", self.music21exception)
    }
}

impl Error for AnalysisException {}

impl Music21ExceptionTrait for AnalysisException {}