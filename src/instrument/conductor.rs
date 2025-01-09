pub struct Conductor {
    instrument: Instrument,
}

impl Conductor {
    pub fn new() -> Conductor {
        Conductor {
            instrument: Instrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
