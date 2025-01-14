use std::error::Error;

use crate::defaults::StringType;

#[derive(Debug)]
pub(crate) struct NoteException {
    music21exception: Music21Exception,
}

impl NoteException {
    pub(crate) fn new(error_message: StringType) -> NoteException {
        NoteException {
            music21exception: Music21Exception::new(),
        }
    }
}

impl Display for NoteException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.music21exception)
    }
}

impl Error for NoteException {}
