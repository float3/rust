pub(crate)  struct ReedOrgan {
    organ: Organ,
}

impl ReedOrgan {
    pub(crate)  fn new() -> ReedOrgan {
        ReedOrgan {
            organ: Organ::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
