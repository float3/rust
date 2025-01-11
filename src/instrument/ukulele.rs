pub(crate) struct Ukulele {
    stringinstrument: StringInstrument,
}

impl Ukulele {
    pub(crate) fn new() -> Ukulele {
        Ukulele {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
