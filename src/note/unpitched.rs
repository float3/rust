pub(crate) struct Unpitched {
    notrest: NotRest,
    // equalityAttributes: ,
}

impl Unpitched {
    pub(crate) fn new() -> Unpitched {
        Unpitched {
            notrest: NotRest::new(),
            // equalityAttributes: todo!(),
        }
    }

    // pub(crate) fn new(displayName: ) {
    //     todo!()
    // }
    pub(crate) fn _repr_internal(&self) {
        todo!()
    }
    pub(crate) fn display_pitch(&self) -> Pitch {
        todo!()
    }
    pub(crate) fn display_name(&self) -> String {
        todo!()
    }
}
