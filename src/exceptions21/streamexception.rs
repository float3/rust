use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct StreamException {
    music21exception: Music21Exception,
}

impl StreamException {
    pub(crate) fn new(error_message: String) -> StreamException {
        StreamException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for StreamException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "StreamException: {}", self.music21exception)
    }
}

impl Error for StreamException {}

impl Music21ExceptionTrait for StreamException {}