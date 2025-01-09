pub struct IntervalException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl IntervalException {
    pub fn new() -> IntervalException {
        IntervalException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}