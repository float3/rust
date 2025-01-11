use std::{error::Error, fmt::Display};

use crate::exceptions21::music21exception::Music21Exception;

#[derive(Debug)]
pub(crate) struct ChordException {
    music21exception: Music21Exception,
}

impl ChordException {
    pub(crate) fn new(error_message: String) -> ChordException {
        ChordException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ChordException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.music21exception)
    }
}

impl Error for ChordException {}
