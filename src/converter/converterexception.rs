pub struct ConverterException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl ConverterException {
    pub fn new() -> ConverterException {
        ConverterException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}