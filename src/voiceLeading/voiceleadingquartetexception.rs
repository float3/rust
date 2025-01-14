pub(crate) struct VoiceLeadingQuartetException {
    music21exception: Music21Exception,
}

impl VoiceLeadingQuartetException {
    pub(crate) fn new() -> VoiceLeadingQuartetException {
        VoiceLeadingQuartetException {
            music21exception: Music21Exception::new(),
        }
    }
    
}