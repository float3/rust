pub(crate) struct Oboe {
    woodwindinstrument: WoodwindInstrument,
}

impl Oboe {
    pub(crate) fn new() -> Oboe {
        Oboe {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
