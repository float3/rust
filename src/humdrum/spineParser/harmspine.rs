pub(crate) struct HarmSpine {
    humdrumspine: HumdrumSpine,
}

impl HarmSpine {
    pub(crate) fn new() -> HarmSpine {
        HarmSpine {
            humdrumspine: HumdrumSpine::new(),
        }
    }
    
    pub(crate) fn parse(&self) {
        todo!()
    }
}