pub struct FretlessBass {
    guitar: Guitar,
}

impl FretlessBass {
    pub fn new() -> FretlessBass {
        FretlessBass {
            guitar: Guitar::new(),
        }
    }

    pub fn new(&self) {
        todo!()
    }
}
