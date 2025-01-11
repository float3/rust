pub(crate) struct Bagpipes {
    woodwindinstrument: WoodwindInstrument,
}

impl Bagpipes {
    pub(crate) fn new() -> Bagpipes {
        Bagpipes {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
