pub struct RelativeCounter {
    collections::counter: collections::Counter,
}

impl RelativeCounter {
    pub fn new() -> RelativeCounter {
        RelativeCounter {
            collections::counter: collections::Counter::new(),
        }
    }
    
    pub fn __iter__(&self) {
        todo!()
    }
    pub fn items(&self) {
        todo!()
    }
    pub fn asProportion(&self) {
        todo!()
    }
    pub fn asPercentage(&self) {
        todo!()
    }
}