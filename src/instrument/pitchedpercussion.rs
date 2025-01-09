pub struct PitchedPercussion {
    percussion: Percussion,
}

impl PitchedPercussion {
    pub fn new() -> PitchedPercussion {
        PitchedPercussion {
            percussion: Percussion::new(),
        }
    }
}
