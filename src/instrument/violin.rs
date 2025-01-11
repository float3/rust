pub(crate) struct Violin {
    stringinstrument: StringInstrument,
}

impl Violin {
    pub(crate) fn new() -> Violin {
        Violin {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
