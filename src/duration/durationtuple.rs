pub struct DurationTuple {
    t::namedtuple: t::NamedTuple,
    type: String,
    dots: i32,
    quarterLength: ,
}

impl DurationTuple {
    pub fn new() -> DurationTuple {
        DurationTuple {
            t::namedtuple: t::NamedTuple::new(),
            type: todo!(),
            dots: todo!(),
            quarterLength: todo!(),
        }
    }
    
    pub fn augmentOrDiminish(&self, amountToScale: ) {
        todo!()
    }
    pub fn ordinal(&self) {
        todo!()
    }
}