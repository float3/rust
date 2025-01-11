pub(crate) struct WoodwindInstrument {
    instrument: Instrument,
}

impl WoodwindInstrument {
    pub(crate) fn new() -> WoodwindInstrument {
        WoodwindInstrument {
            instrument: Instrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
