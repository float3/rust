pub(crate) struct Sampler {
    keyboardinstrument: KeyboardInstrument,
}

impl Sampler {
    pub(crate) fn new() -> Sampler {
        Sampler {
            keyboardinstrument: KeyboardInstrument::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
