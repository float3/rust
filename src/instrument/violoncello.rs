pub struct Violoncello {
    stringinstrument: StringInstrument,
}

impl Violoncello {
    pub fn new() -> Violoncello {
        Violoncello {
            stringinstrument: StringInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
