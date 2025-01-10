pub(crate)  struct Trombone {
    brassinstrument: BrassInstrument,
}

impl Trombone {
    pub(crate)  fn new() -> Trombone {
        Trombone {
            brassinstrument: BrassInstrument::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
