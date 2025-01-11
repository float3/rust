pub(crate) struct Banjo {
    stringinstrument: StringInstrument,
}

impl Banjo {
    pub(crate) fn new() -> Banjo {
        Banjo {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
