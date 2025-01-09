pub struct Violin {
    stringinstrument: StringInstrument,
}

impl Violin {
    pub fn new() -> Violin {
        Violin {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
