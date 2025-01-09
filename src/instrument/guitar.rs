pub struct Guitar {
    stringinstrument: StringInstrument,
}

impl Guitar {
    pub fn new() -> Guitar {
        Guitar {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
