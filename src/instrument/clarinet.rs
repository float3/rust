pub struct Clarinet {
    woodwindinstrument: WoodwindInstrument,
}

impl Clarinet {
    pub fn new() -> Clarinet {
        Clarinet {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
