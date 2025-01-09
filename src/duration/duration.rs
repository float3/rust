pub struct Duration {
    prebase::protom21object: prebase::ProtoM21Object,
    isGrace: bool,
    __slots__: ,

    linked: ,
    quarterLength: ,
}

impl Duration {
    pub fn new() -> Duration {
        Duration {
            prebase::protom21object: prebase::ProtoM21Object::new(),
            isGrace: todo!(),
            __slots__: todo!(),
            _DOC_ATTR: todo!(),
            linked: todo!(),
            quarterLength: todo!(),
        }
    }
    
    pub fn new(&self) {
        todo!()
    }
    pub fn __eq__(&self, other: ) {
        todo!()
    }
    pub fn _reprInternal(&self) {
        todo!()
    }
    pub fn __deepcopy__(&self, memo: ) {
        todo!()
    }
    pub fn _updateComponents(&self) {
        todo!()
    }
    pub fn _getLinked(&self) -> bool {
        todo!()
    }
    pub fn _setLinked(&self, value: bool) {
        todo!()
    }
    pub fn addDurationTuple(&self, dur: ) {
        todo!()
    }
    pub fn appendTuplet(&self, newTuplet: Tuplet) {
        todo!()
    }
    pub fn augmentOrDiminish(&self, amountToScale: ) {
        todo!()
    }
    pub fn clear(&self) {
        todo!()
    }
    pub fn componentIndexAtQtrPosition(&self, quarterPosition: ) {
        todo!()
    }
    pub fn componentStartTime(&self, componentIndex: i32) -> f64 {
        todo!()
    }
    pub fn consolidate(&self) {
        todo!()
    }
    pub fn getGraceDuration(&self, appoggiatura: ) {
        todo!()
    }
    pub fn informClient(&self) -> bool {
        todo!()
    }
    pub fn sliceComponentAtPosition(&self, quarterPosition: ) {
        todo!()
    }
    pub fn currentComponents(&self) {
        todo!()
    }
    pub fn splitDotGroups(&self) {
        todo!()
    }
    pub fn _updateQuarterLength(&self) {
        todo!()
    }
    pub fn components(&self) {
        todo!()
    }
    pub fn components(&self, value: ) {
        todo!()
    }
    pub fn dotGroups(&self) {
        todo!()
    }
    pub fn dotGroups(&self, value: ) {
        todo!()
    }
    pub fn dots(&self) -> i32 {
        todo!()
    }
    pub fn dots(&self, value: i32) {
        todo!()
    }
    pub fn fullName(&self) -> String {
        todo!()
    }
    pub fn isComplex(&self) -> bool {
        todo!()
    }
    pub fn ordinal(&self) {
        todo!()
    }
    pub fn quarterLengthNoTuplets(&self) -> f64 {
        todo!()
    }
    pub fn _getQuarterLength(&self) {
        todo!()
    }
    pub fn _setQuarterLength(&self, value: ) {
        todo!()
    }
    pub fn tuplets(&self) {
        todo!()
    }
    pub fn tuplets(&self, tupletTuple: ) {
        todo!()
    }
    pub fn aggregateTupletMultiplier(&self) {
        todo!()
    }
    pub fn type(&self) -> String {
        todo!()
    }
    pub fn type(&self, value: String) {
        todo!()
    }
}