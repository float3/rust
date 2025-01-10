pub(crate)  struct Vocalist {
    instrument: Instrument,
}

impl Vocalist {
    pub(crate)  fn new() -> Vocalist {
        Vocalist {
            instrument: Instrument::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
