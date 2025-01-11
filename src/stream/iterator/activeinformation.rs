pub(crate) struct ActiveInformation {
    t::typeddict: t::TypedDict,
    stream: ,
    elementIndex: IntegerType,
    iterSection: ,
    sectionIndex: IntegerType,
    lastYielded: ,
}

impl ActiveInformation {
    pub(crate) fn new() -> ActiveInformation {
        ActiveInformation {
            t::typeddict: t::TypedDict::new(),
            stream: todo!(),
            elementIndex: todo!(),
            iterSection: todo!(),
            sectionIndex: todo!(),
            lastYielded: todo!(),
        }
    }
    
}