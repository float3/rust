pub(crate) struct LilyTranslateException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl LilyTranslateException {
    pub(crate) fn new() -> LilyTranslateException {
        LilyTranslateException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}