pub struct KeyboardInstrument {
    instrument: Instrument,
}

impl KeyboardInstrument {
    pub fn new() -> KeyboardInstrument {
        KeyboardInstrument {
            instrument: Instrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
