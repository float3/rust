pub(crate)  struct ConverterFileException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ConverterFileException {
    pub(crate)  fn new() -> ConverterFileException {
        ConverterFileException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}