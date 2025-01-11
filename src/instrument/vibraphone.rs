pub(crate) struct Vibraphone {
    pitchedpercussion: PitchedPercussion,
}

impl Vibraphone {
    pub(crate) fn new() -> Vibraphone {
        Vibraphone {
            pitchedpercussion: PitchedPercussion::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
