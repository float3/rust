pub(crate) struct Piano {
    keyboardinstrument: KeyboardInstrument,
}

impl Piano {
    pub(crate) fn new() -> Piano {
        Piano {
            keyboardinstrument: KeyboardInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
