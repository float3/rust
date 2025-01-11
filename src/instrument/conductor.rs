pub(crate) struct Conductor {
    instrument: Instrument,
}

impl Conductor {
    pub(crate) fn new() -> Conductor {
        Conductor {
            instrument: Instrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
