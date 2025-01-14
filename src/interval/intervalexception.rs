use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct IntervalException {
    music21exception: Music21Exception,
}

impl IntervalException {
    pub(crate) fn new(error_message: String) -> IntervalException {
        IntervalException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for IntervalException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "IntervalException: {}", self.music21exception)
    }
}

impl Error for IntervalException {}

impl Music21ExceptionTrait for IntervalException {}