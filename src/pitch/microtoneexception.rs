pub(crate) struct MicrotoneException {
    music21exception: Music21Exception,
}

impl MicrotoneException {
    pub(crate) fn new() -> MicrotoneException {
        MicrotoneException {
            music21exception: Music21Exception::new(),
        }
    }
}
