pub(crate)  struct ElectricPiano {
    piano: Piano,
}

impl ElectricPiano {
    pub(crate)  fn new() -> ElectricPiano {
        ElectricPiano {
            piano: Piano::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
