pub(crate)  struct Bassoon {
    woodwindinstrument: WoodwindInstrument,
}

impl Bassoon {
    pub(crate)  fn new() -> Bassoon {
        Bassoon {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
