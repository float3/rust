pub(crate)  struct Mandolin {
    stringinstrument: StringInstrument,
}

impl Mandolin {
    pub(crate)  fn new() -> Mandolin {
        Mandolin {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
