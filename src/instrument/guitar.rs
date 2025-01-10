pub(crate)  struct Guitar {
    stringinstrument: StringInstrument,
}

impl Guitar {
    pub(crate)  fn new() -> Guitar {
        Guitar {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
