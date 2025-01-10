pub(crate)  struct Ocarina {
    flute: Flute,
}

impl Ocarina {
    pub(crate)  fn new() -> Ocarina {
        Ocarina {
            flute: Flute::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
