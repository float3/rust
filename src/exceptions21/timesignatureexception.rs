pub(crate) struct TimeSignatureException {
    meterexception: MeterException,
}

impl TimeSignatureException {
    pub(crate) fn new() -> TimeSignatureException {
        TimeSignatureException {
            meterexception: MeterException::new(),
        }
    }
    
}