use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct MetadataException {
    music21exception: Music21Exception,
}

impl MetadataException {
    pub(crate) fn new(error_message: String) -> MetadataException {
        MetadataException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for MetadataException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MetadataException: {}", self.music21exception)
    }
}

impl Error for MetadataException {}

impl Music21ExceptionTrait for MetadataException {}