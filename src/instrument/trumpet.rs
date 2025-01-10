pub(crate)  struct Trumpet {
    brassinstrument: BrassInstrument,
}

impl Trumpet {
    pub(crate)  fn new() -> Trumpet {
        Trumpet {
            brassinstrument: BrassInstrument::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
