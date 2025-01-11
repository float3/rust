pub(crate) struct KeyboardInstrument {
    instrument: Instrument,
}

impl KeyboardInstrument {
    pub(crate) fn new() -> KeyboardInstrument {
        KeyboardInstrument {
            instrument: Instrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
