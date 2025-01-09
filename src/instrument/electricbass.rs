pub struct ElectricBass {
    guitar: Guitar,
}

impl ElectricBass {
    pub fn new() -> ElectricBass {
        ElectricBass {
            guitar: Guitar::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
