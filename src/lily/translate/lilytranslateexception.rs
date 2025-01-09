pub struct LilyTranslateException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl LilyTranslateException {
    pub fn new() -> LilyTranslateException {
        LilyTranslateException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}