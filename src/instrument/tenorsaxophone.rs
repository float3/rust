pub(crate)  struct TenorSaxophone {
    saxophone: Saxophone,
}

impl TenorSaxophone {
    pub(crate)  fn new() -> TenorSaxophone {
        TenorSaxophone {
            saxophone: Saxophone::new(),
        }
    }

    pub(crate)  fn new(&self) {
        todo!()
    }
}
