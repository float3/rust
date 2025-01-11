pub(crate) struct Flute {
    woodwindinstrument: WoodwindInstrument,
}

impl Flute {
    pub(crate) fn new() -> Flute {
        Flute {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
