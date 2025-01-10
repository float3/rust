pub(crate)  struct AcousticBass {
    guitar: Guitar,
}

impl AcousticBass {
    pub(crate)  fn new() -> AcousticBass {
        AcousticBass {
            guitar: Guitar::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
