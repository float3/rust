pub(crate) struct Contrabassoon {
    bassoon: Bassoon,
}

impl Contrabassoon {
    pub(crate) fn new() -> Contrabassoon {
        Contrabassoon {
            bassoon: Bassoon::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
