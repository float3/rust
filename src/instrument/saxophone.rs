pub(crate) struct Saxophone {
    woodwindinstrument: WoodwindInstrument,
}

impl Saxophone {
    pub(crate) fn new() -> Saxophone {
        Saxophone {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
