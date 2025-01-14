use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct NChordLinearSegmentException {
    music21exception: Music21Exception,
}

impl NChordLinearSegmentException {
    pub(crate) fn new(error_message: String) -> NChordLinearSegmentException {
        NChordLinearSegmentException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for NChordLinearSegmentException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "NChordLinearSegmentException: {}", self.music21exception)
    }
}

impl Error for NChordLinearSegmentException {}

impl Music21ExceptionTrait for NChordLinearSegmentException {}