pub struct AcousticGuitar {
    guitar: Guitar,
}

impl AcousticGuitar {
    pub fn new() -> AcousticGuitar {
        AcousticGuitar {
            guitar: Guitar::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
