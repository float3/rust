use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct VexflowToM21JException {
    music21exception: Music21Exception,
}

impl VexflowToM21JException {
    pub(crate) fn new(error_message: String) -> VexflowToM21JException {
        VexflowToM21JException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for VexflowToM21JException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "VexflowToM21JException: {}", self.music21exception)
    }
}

impl Error for VexflowToM21JException {}

impl Music21ExceptionTrait for VexflowToM21JException {}