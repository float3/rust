pub struct GenericInterval {
    intervalbase: IntervalBase,
}

impl GenericInterval {
    pub fn new() -> GenericInterval {
        GenericInterval {
            intervalbase: IntervalBase::new(),
        }
    }
    
    pub fn new(&self, value: ) {
        todo!()
    }
    pub fn _reprInternal(&self) {
        todo!()
    }
    pub fn __eq__(&self, other: ) {
        todo!()
    }
    pub fn __hash__(&self) {
        todo!()
    }
    pub fn value(&self) -> i32 {
        todo!()
    }
    pub fn value(&self, newValue: i32) {
        todo!()
    }
    pub fn directed(&self) -> i32 {
        todo!()
    }
    pub fn directed(&self, newValue: i32) {
        todo!()
    }
    pub fn undirected(&self) -> i32 {
        todo!()
    }
    pub fn direction(&self) -> Direction {
        todo!()
    }
    pub fn isSkip(&self) -> bool {
        todo!()
    }
    pub fn isDiatonicStep(&self) -> bool {
        todo!()
    }
    pub fn isStep(&self) -> bool {
        todo!()
    }
    pub fn isUnison(&self) -> bool {
        todo!()
    }
    pub fn _simpleStepsAndOctaves(&self) {
        todo!()
    }
    pub fn simpleUndirected(&self) -> i32 {
        todo!()
    }
    pub fn semiSimpleUndirected(&self) -> i32 {
        todo!()
    }
    pub fn undirectedOctaves(&self) -> i32 {
        todo!()
    }
    pub fn octaves(&self) -> i32 {
        todo!()
    }
    pub fn simpleDirected(&self) -> i32 {
        todo!()
    }
    pub fn semiSimpleDirected(&self) -> i32 {
        todo!()
    }
    pub fn perfectable(&self) -> bool {
        todo!()
    }
    pub fn _nameFromInt(&self, keyVal: i32) -> String {
        todo!()
    }
    pub fn niceName(&self) -> String {
        todo!()
    }
    pub fn simpleNiceName(&self) -> String {
        todo!()
    }
    pub fn semiSimpleNiceName(&self) -> String {
        todo!()
    }
    pub fn staffDistance(&self) -> i32 {
        todo!()
    }
    pub fn mod7inversion(&self) -> i32 {
        todo!()
    }
    pub fn mod7(&self) -> i32 {
        todo!()
    }
    pub fn complement(&self) -> GenericInterval {
        todo!()
    }
    pub fn reverse(&self) -> GenericInterval {
        todo!()
    }
    pub fn transposePitch(&self, p: ) {
        todo!()
    }
    pub fn transposePitchKeyAware(&self, p: ) {
        todo!()
    }
    pub fn getDiatonic(&self, specifier: ) -> DiatonicInterval {
        todo!()
    }
}