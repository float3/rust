pub(crate)  struct VoiceLeadingQuartetException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl VoiceLeadingQuartetException {
    pub(crate)  fn new() -> VoiceLeadingQuartetException {
        VoiceLeadingQuartetException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}