pub struct Harpsichord {
    keyboardinstrument: KeyboardInstrument,
}

impl Harpsichord {
    pub fn new() -> Harpsichord {
        Harpsichord {
            keyboardinstrument: KeyboardInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
