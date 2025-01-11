pub(crate) struct SopranoSaxophone {
    saxophone: Saxophone,
}

impl SopranoSaxophone {
    pub(crate) fn new() -> SopranoSaxophone {
        SopranoSaxophone {
            saxophone: Saxophone::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
