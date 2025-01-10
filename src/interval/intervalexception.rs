use crate::exceptions21::music21exception::Music21Exception;
use std::{error::Error, fmt::Display};

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

    pub(crate) fn error_message(&self) -> String {
        self.music21exception.error_message()
    }
}

impl Display for IntervalException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.music21exception)
    }
}

impl Error for IntervalException {}
