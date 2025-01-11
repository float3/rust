pub(crate) struct BassDrum {
    unpitchedpercussion: UnpitchedPercussion,
}

impl BassDrum {
    pub(crate) fn new() -> BassDrum {
        BassDrum {
            unpitchedpercussion: UnpitchedPercussion::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
