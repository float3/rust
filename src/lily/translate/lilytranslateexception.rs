use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct LilyTranslateException {
    music21exception: Music21Exception,
}

impl LilyTranslateException {
    pub(crate) fn new(error_message: String) -> LilyTranslateException {
        LilyTranslateException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for LilyTranslateException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LilyTranslateException: {}", self.music21exception)
    }
}

impl Error for LilyTranslateException {}

impl Music21ExceptionTrait for LilyTranslateException {}