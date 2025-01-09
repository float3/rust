pub struct Saxophone {
    woodwindinstrument: WoodwindInstrument,
}

impl Saxophone {
    pub fn new() -> Saxophone {
        Saxophone {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
