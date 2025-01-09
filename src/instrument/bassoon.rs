pub struct Bassoon {
    woodwindinstrument: WoodwindInstrument,
}

impl Bassoon {
    pub fn new() -> Bassoon {
        Bassoon {
            woodwindinstrument: WoodwindInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
