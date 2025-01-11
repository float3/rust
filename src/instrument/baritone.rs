pub(crate) struct Baritone {
    vocalist: Vocalist,
}

impl Baritone {
    pub(crate) fn new() -> Baritone {
        Baritone {
            vocalist: Vocalist::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
