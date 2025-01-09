pub struct Piano {
    keyboardinstrument: KeyboardInstrument,
}

impl Piano {
    pub fn new() -> Piano {
        Piano {
            keyboardinstrument: KeyboardInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
