use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use crate::{
    defaults::StringType,
    exceptions21::music21exception::{Music21Exception, Music21ExceptionTrait},
};

#[derive(Debug)]
pub(crate) struct ChordException {
    music21exception: Music21Exception,
}

impl ChordException {
    pub(crate) fn new(error_message: StringType) -> ChordException {
        ChordException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ChordException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.music21exception)
    }
}

impl Error for ChordException {}

impl Music21ExceptionTrait for ChordException {}
