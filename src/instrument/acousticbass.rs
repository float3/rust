pub struct AcousticBass {
    guitar: Guitar,
}

impl AcousticBass {
    pub fn new() -> AcousticBass {
        AcousticBass {
            guitar: Guitar::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
