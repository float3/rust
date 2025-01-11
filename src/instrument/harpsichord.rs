pub(crate) struct Harpsichord {
    keyboardinstrument: KeyboardInstrument,
}

impl Harpsichord {
    pub(crate) fn new() -> Harpsichord {
        Harpsichord {
            keyboardinstrument: KeyboardInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
