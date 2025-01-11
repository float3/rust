pub(crate) struct Violoncello {
    stringinstrument: StringInstrument,
}

impl Violoncello {
    pub(crate) fn new() -> Violoncello {
        Violoncello {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
