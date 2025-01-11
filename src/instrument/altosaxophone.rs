pub(crate) struct AltoSaxophone {
    saxophone: Saxophone,
}

impl AltoSaxophone {
    pub(crate) fn new() -> AltoSaxophone {
        AltoSaxophone {
            saxophone: Saxophone::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
