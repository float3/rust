pub(crate) struct Harp {
    stringinstrument: StringInstrument,
}

impl Harp {
    pub(crate) fn new() -> Harp {
        Harp {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
