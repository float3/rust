pub(crate) struct Clavichord {
    keyboardinstrument: KeyboardInstrument,
}

impl Clavichord {
    pub(crate) fn new() -> Clavichord {
        Clavichord {
            keyboardinstrument: KeyboardInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
