use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct ImmutableStreamException {
    music21exception: Music21Exception,
}

impl ImmutableStreamException {
    pub(crate) fn new(error_message: String) -> ImmutableStreamException {
        ImmutableStreamException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for ImmutableStreamException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ImmutableStreamException: {}", self.music21exception)
    }
}

impl Error for ImmutableStreamException {}

impl Music21ExceptionTrait for ImmutableStreamException {}