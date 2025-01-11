pub(crate) struct BrassInstrument {
    instrument: Instrument,
}

impl BrassInstrument {
    pub(crate) fn new() -> BrassInstrument {
        BrassInstrument {
            instrument: Instrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
