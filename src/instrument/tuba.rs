pub(crate) struct Tuba {
    brassinstrument: BrassInstrument,
}

impl Tuba {
    pub(crate) fn new() -> Tuba {
        Tuba {
            brassinstrument: BrassInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
