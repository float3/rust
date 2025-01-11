pub(crate) struct RelativeCounter {
    collections::counter: collections::Counter,
}

impl RelativeCounter {
    pub(crate) fn new() -> RelativeCounter {
        RelativeCounter {
            collections::counter: collections::Counter::new(),
        }
    }
    
    pub(crate) fn __iter__(&self) {
        todo!()
    }
    pub(crate) fn items(&self) {
        todo!()
    }
    pub(crate) fn asProportion(&self) {
        todo!()
    }
    pub(crate) fn asPercentage(&self) {
        todo!()
    }
}