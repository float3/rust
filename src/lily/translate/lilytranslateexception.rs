pub(crate) struct LilyTranslateException {
    music21exception: Music21Exception,
}

impl LilyTranslateException {
    pub(crate) fn new() -> LilyTranslateException {
        LilyTranslateException {
            music21exception: Music21Exception::new(),
        }
    }
    
}