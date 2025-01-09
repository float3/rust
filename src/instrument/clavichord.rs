pub struct Clavichord {
    keyboardinstrument: KeyboardInstrument,
}

impl Clavichord {
    pub fn new() -> Clavichord {
        Clavichord {
            keyboardinstrument: KeyboardInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
