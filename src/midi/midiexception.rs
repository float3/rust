use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct MidiException {
    music21exception: Music21Exception,
}

impl MidiException {
    pub(crate) fn new(error_message: String) -> MidiException {
        MidiException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for MidiException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MidiException: {}", self.music21exception)
    }
}

impl Error for MidiException {}

impl Music21ExceptionTrait for MidiException {}