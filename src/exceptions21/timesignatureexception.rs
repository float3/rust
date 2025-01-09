pub struct TimeSignatureException {
    meterexception: MeterException,
}

impl TimeSignatureException {
    pub fn new() -> TimeSignatureException {
        TimeSignatureException {
            meterexception: MeterException::new(),
        }
    }
    
}