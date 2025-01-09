pub struct Trumpet {
    brassinstrument: BrassInstrument,
}

impl Trumpet {
    pub fn new() -> Trumpet {
        Trumpet {
            brassinstrument: BrassInstrument::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
