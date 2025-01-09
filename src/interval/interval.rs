pub struct Interval {
    intervalbase: IntervalBase,
}

impl Interval {
    pub fn new() -> Interval {
        Interval {
            intervalbase: IntervalBase::new(),
        }
    }
    
    pub fn new(&self) {
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
    pub fn generic(&self) -> GenericInterval {
        todo!()
    }
    pub fn name(&self) -> String {
        todo!()
    }
    pub fn niceName(&self) -> String {
        todo!()
    }
    pub fn simpleName(&self) -> String {
        todo!()
    }
    pub fn simpleNiceName(&self) -> String {
        todo!()
    }
    pub fn semiSimpleName(&self) -> String {
        todo!()
    }
    pub fn semiSimpleNiceName(&self) -> String {
        todo!()
    }
    pub fn directedName(&self) -> String {
        todo!()
    }
    pub fn directedNiceName(&self) -> String {
        todo!()
    }
    pub fn directedSimpleName(&self) -> String {
        todo!()
    }
    pub fn directedSimpleNiceName(&self) -> String {
        todo!()
    }
    pub fn semitones(&self) {
        todo!()
    }
    pub fn direction(&self) {
        todo!()
    }
    pub fn specifier(&self) {
        todo!()
    }
    pub fn specificName(&self) -> String {
        todo!()
    }
    pub fn isDiatonicStep(&self) -> bool {
        todo!()
    }
    pub fn isChromaticStep(&self) -> bool {
        todo!()
    }
    pub fn isStep(&self) -> bool {
        todo!()
    }
    pub fn isSkip(&self) -> bool {
        todo!()
    }
    pub fn isConsonant(&self) -> bool {
        todo!()
    }
    pub fn complement(&self) -> Interval {
        todo!()
    }
    pub fn intervalClass(&self) -> i32 {
        todo!()
    }
    pub fn cents(&self) -> f64 {
        todo!()
    }
    pub fn _diatonicIntervalCentShift(&self) -> f64 {
        todo!()
    }
    pub fn transposePitch(&self, p: ) {
        todo!()
    }
    pub fn _diatonicTransposePitch(&self, p: ) {
        todo!()
    }
    pub fn reverse(&self) {
        todo!()
    }
    pub fn pitchStart(&self) {
        todo!()
    }
    pub fn pitchStart(&self, p: ) {
        todo!()
    }
    pub fn pitchEnd(&self) {
        todo!()
    }
    pub fn pitchEnd(&self, p: ) {
        todo!()
    }
    pub fn noteStart(&self) {
        todo!()
    }
    pub fn noteStart(&self, n: ) {
        todo!()
    }
    pub fn noteEnd(&self) {
        todo!()
    }
    pub fn noteEnd(&self, n: ) {
        todo!()
    }
}