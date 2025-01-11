pub(crate) struct SuspendedCymbal {
    cymbals: Cymbals,
}

impl SuspendedCymbal {
    pub(crate) fn new() -> SuspendedCymbal {
        SuspendedCymbal {
            cymbals: Cymbals::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
