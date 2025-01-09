pub struct VexflowToM21JException {
    music21exception: Music21Exception,
}

impl VexflowToM21JException {
    pub fn new() -> VexflowToM21JException {
        VexflowToM21JException {
            music21exception: Music21Exception::new(),
        }
    }
}
