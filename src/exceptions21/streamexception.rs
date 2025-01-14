pub(crate) struct StreamException {
    music21exception: Music21Exception,
}

impl StreamException {
    pub(crate) fn new() -> StreamException {
        StreamException {
            music21exception: Music21Exception::new(),
        }
    }
}

impl Music21Exception for StreamException {}
