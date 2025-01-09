pub struct Bagpipes {
    woodwindinstrument: WoodwindInstrument,
}

impl Bagpipes {
    pub fn new() -> Bagpipes {
        Bagpipes {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
