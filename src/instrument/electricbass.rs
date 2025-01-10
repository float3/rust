pub(crate)  struct ElectricBass {
    guitar: Guitar,
}

impl ElectricBass {
    pub(crate)  fn new() -> ElectricBass {
        ElectricBass {
            guitar: Guitar::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
