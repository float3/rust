pub struct Banjo {
    stringinstrument: StringInstrument,
}

impl Banjo {
    pub fn new() -> Banjo {
        Banjo {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
