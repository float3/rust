pub(crate) struct PitchException {
    music21exception: Music21Exception,
}

impl PitchException {
    pub(crate) fn new() -> PitchException {
        PitchException {
            music21exception: Music21Exception::new(),
        }
    }
}
