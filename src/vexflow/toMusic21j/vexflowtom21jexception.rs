pub(crate)  struct VexflowToM21JException {
    music21exception: Music21Exception,
}

impl VexflowToM21JException {
    pub(crate)  fn new() -> VexflowToM21JException {
        VexflowToM21JException {
            music21exception: Music21Exception::new(),
        }
    }
}
