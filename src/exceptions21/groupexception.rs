pub(crate) struct GroupException {
    music21exception: Music21Exception,
}

impl GroupException {
    pub(crate) fn new() -> GroupException {
        GroupException {
            music21exception: Music21Exception::new(),
        }
    }
}
