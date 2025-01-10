pub(crate)  struct Harmonica {
    instrument: Instrument,
}

impl Harmonica {
    pub(crate)  fn new() -> Harmonica {
        Harmonica {
            instrument: Instrument::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
