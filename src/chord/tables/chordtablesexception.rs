use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ChordTablesException {
    music21exception: Music21Exception,
}

impl ChordTablesException {
    pub(crate) fn new(error_message: String) -> ChordTablesException {
        ChordTablesException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ChordTablesException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ChordTablesException: {}", self.music21exception)
    }
}

impl Error for ChordTablesException {}

impl Music21ExceptionTrait for ChordTablesException {}