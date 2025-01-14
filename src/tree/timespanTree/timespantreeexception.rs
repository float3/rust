use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct TimespanTreeException {
    music21exception: Music21Exception,
}

impl TimespanTreeException {
    pub(crate) fn new(error_message: String) -> TimespanTreeException {
        TimespanTreeException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for TimespanTreeException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TimespanTreeException: {}", self.music21exception)
    }
}

impl Error for TimespanTreeException {}

impl Music21ExceptionTrait for TimespanTreeException {}