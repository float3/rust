pub(crate)  struct PartStaff {
    part: Part,
}

impl PartStaff {
    pub(crate)  fn new() -> PartStaff {
        PartStaff { part: Part::new() }
    }
}
