pub(crate) struct AccidentalException {
    music21exception: Music21Exception,
}

impl AccidentalException {
    pub(crate) fn new() -> AccidentalException {
        AccidentalException {
            music21exception: Music21Exception::new(),
        }
    }
}
