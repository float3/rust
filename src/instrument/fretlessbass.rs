pub(crate)  struct FretlessBass {
    guitar: Guitar,
}

impl FretlessBass {
    pub(crate)  fn new() -> FretlessBass {
        FretlessBass {
            guitar: Guitar::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
