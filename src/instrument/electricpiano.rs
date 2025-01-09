pub struct ElectricPiano {
    piano: Piano,
}

impl ElectricPiano {
    pub fn new() -> ElectricPiano {
        ElectricPiano {
            piano: Piano::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
