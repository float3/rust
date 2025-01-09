pub struct ElectricGuitar {
    guitar: Guitar,
}

impl ElectricGuitar {
    pub fn new() -> ElectricGuitar {
        ElectricGuitar {
            guitar: Guitar::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
