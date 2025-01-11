pub(crate) struct AcousticGuitar {
    guitar: Guitar,
}

impl AcousticGuitar {
    pub(crate) fn new() -> AcousticGuitar {
        AcousticGuitar {
            guitar: Guitar::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
