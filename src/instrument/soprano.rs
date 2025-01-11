pub(crate) struct Soprano {
    vocalist: Vocalist,
}

impl Soprano {
    pub(crate) fn new() -> Soprano {
        Soprano {
            vocalist: Vocalist::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
