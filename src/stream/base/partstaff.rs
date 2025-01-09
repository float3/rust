pub struct PartStaff {
    part: Part,
}

impl PartStaff {
    pub fn new() -> PartStaff {
        PartStaff { part: Part::new() }
    }
}
