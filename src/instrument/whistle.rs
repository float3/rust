pub(crate) struct Whistle {
    flute: Flute,
}

impl Whistle {
    pub(crate) fn new() -> Whistle {
        Whistle {
            flute: Flute::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
