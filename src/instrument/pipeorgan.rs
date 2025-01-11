pub(crate) struct PipeOrgan {
    organ: Organ,
}

impl PipeOrgan {
    pub(crate) fn new() -> PipeOrgan {
        PipeOrgan {
            organ: Organ::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
