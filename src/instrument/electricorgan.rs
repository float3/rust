pub(crate) struct ElectricOrgan {
    organ: Organ,
}

impl ElectricOrgan {
    pub(crate) fn new() -> ElectricOrgan {
        ElectricOrgan {
            organ: Organ::new(),
        }
    }

    pub(crate) fn new(&self) {
        todo!()
    }
}
