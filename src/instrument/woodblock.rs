pub(crate) struct Woodblock {
    unpitchedpercussion: UnpitchedPercussion,
}

impl Woodblock {
    pub(crate) fn new() -> Woodblock {
        Woodblock {
            unpitchedpercussion: UnpitchedPercussion::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
