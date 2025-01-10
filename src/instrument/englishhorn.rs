pub(crate)  struct EnglishHorn {
    woodwindinstrument: WoodwindInstrument,
}

impl EnglishHorn {
    pub(crate)  fn new() -> EnglishHorn {
        EnglishHorn {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
