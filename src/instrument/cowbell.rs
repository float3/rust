pub(crate)  struct Cowbell {
    unpitchedpercussion: UnpitchedPercussion,
}

impl Cowbell {
    pub(crate)  fn new() -> Cowbell {
        Cowbell {
            unpitchedpercussion: UnpitchedPercussion::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
