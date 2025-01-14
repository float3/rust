use crate::exceptions21::music21exception::{
    Music21Exception, Music21ExceptionTrait
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub(crate) struct NoteException {
    music21exception: Music21Exception,
}

impl NoteException {
    pub(crate) fn new(error_message: String) -> NoteException {
        NoteException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}

impl Display for NoteException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "NoteException: {}", self.music21exception)
    }
}

impl Error for NoteException {}

impl Music21ExceptionTrait for NoteException {}