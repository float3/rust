use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct VoiceLeadingQuartetException {
    music21exception: Music21Exception,
}

impl VoiceLeadingQuartetException {
    pub(crate) fn new(error_message: String) -> VoiceLeadingQuartetException {
        VoiceLeadingQuartetException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for VoiceLeadingQuartetException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "VoiceLeadingQuartetException: {}", self.music21exception)
    }
}

impl Error for VoiceLeadingQuartetException {}

impl Music21ExceptionTrait for VoiceLeadingQuartetException {}