pub(crate) struct DurationTuple {
    t::namedtuple: t::NamedTuple,
    type: String,
    dots: IntegerType,
    quarterLength: ,
}

impl DurationTuple {
    pub(crate) fn new() -> DurationTuple {
        DurationTuple {
            t::namedtuple: t::NamedTuple::new(),
            type: todo!(),
            dots: todo!(),
            quarterLength: todo!(),
        }
    }
    
    pub(crate) fn augmentOrDiminish(&self, amountToScale: ) {
        todo!()
    }
    pub(crate) fn ordinal(&self) {
        todo!()
    }
}