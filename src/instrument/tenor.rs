pub(crate) struct Tenor {
    vocalist: Vocalist,
}

impl Tenor {
    pub(crate) fn new() -> Tenor {
        Tenor {
            vocalist: Vocalist::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
