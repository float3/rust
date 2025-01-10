use crate::exceptions21::music21exception::Music21Exception;

pub(crate)  struct Music21ObjectException {
    music21exception: Music21Exception,
}

impl Music21ObjectException {
    pub(crate)  fn new(error_message: String) -> Music21ObjectException {
        Music21ObjectException {
            music21exception: Music21Exception::new(error_message),
        }
    }
}
