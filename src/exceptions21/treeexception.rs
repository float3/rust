use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct TreeException {
    music21exception: Music21Exception,
}

impl TreeException {
    pub(crate) fn new(error_message: String) -> TreeException {
        TreeException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for TreeException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TreeException: {}", self.music21exception)
    }
}

impl Error for TreeException {}

impl Music21ExceptionTrait for TreeException {}