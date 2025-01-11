pub(crate) struct Organ {
    instrument: Instrument,
}

impl Organ {
    pub(crate) fn new() -> Organ {
        Organ {
            instrument: Instrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
