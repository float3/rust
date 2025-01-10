pub(crate)  struct Choir {
    vocalist: Vocalist,
}

impl Choir {
    pub(crate)  fn new() -> Choir {
        Choir {
            vocalist: Vocalist::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
