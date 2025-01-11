pub(crate) struct Shamisen {
    stringinstrument: StringInstrument,
}

impl Shamisen {
    pub(crate) fn new() -> Shamisen {
        Shamisen {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
