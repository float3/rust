pub(crate) struct BaritoneSaxophone {
    saxophone: Saxophone,
}

impl BaritoneSaxophone {
    pub(crate) fn new() -> BaritoneSaxophone {
        BaritoneSaxophone {
            saxophone: Saxophone::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
