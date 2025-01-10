pub(crate)  struct Clarinet {
    woodwindinstrument: WoodwindInstrument,
}

impl Clarinet {
    pub(crate)  fn new() -> Clarinet {
        Clarinet {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
