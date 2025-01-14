use crate::{defaults::StringType, exceptions21::music21exception::Music21Exception};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub(crate) struct NotRestException {
    music21exception: Music21Exception,
}

impl NotRestException {
    pub(crate) fn new(error_message: StringType) -> NotRestException {
        NotRestException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for NotRestException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.music21exception)
    }
}

impl Error for NotRestException {}
