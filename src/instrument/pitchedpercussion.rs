pub(crate) struct PitchedPercussion {
    percussion: Percussion,
}

impl PitchedPercussion {
    pub(crate) fn new() -> PitchedPercussion {
        PitchedPercussion {
            percussion: Percussion::new(),
        }
    }
}
