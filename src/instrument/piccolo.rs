pub(crate)  struct Piccolo {
    flute: Flute,
}

impl Piccolo {
    pub(crate)  fn new() -> Piccolo {
        Piccolo {
            flute: Flute::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
