pub(crate)  struct ElectricGuitar {
    guitar: Guitar,
}

impl ElectricGuitar {
    pub(crate)  fn new() -> ElectricGuitar {
        ElectricGuitar {
            guitar: Guitar::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
