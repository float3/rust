use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ThreeNoteLinearSegmentException {
    music21exception: Music21Exception,
}

impl ThreeNoteLinearSegmentException {
    pub(crate) fn new(error_message: String) -> ThreeNoteLinearSegmentException {
        ThreeNoteLinearSegmentException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ThreeNoteLinearSegmentException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ThreeNoteLinearSegmentException: {}", self.music21exception)
    }
}

impl Error for ThreeNoteLinearSegmentException {}

impl Music21ExceptionTrait for ThreeNoteLinearSegmentException {}