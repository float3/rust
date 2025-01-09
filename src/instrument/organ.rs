pub struct Organ {
    instrument: Instrument,
}

impl Organ {
    pub fn new() -> Organ {
        Organ {
            instrument: Instrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
