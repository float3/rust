pub struct ChromaticInterval {
    intervalbase: IntervalBase,
}

impl ChromaticInterval {
    pub fn new() -> ChromaticInterval {
        ChromaticInterval {
            intervalbase: IntervalBase::new(),
        }
    }
    
    pub fn new(&self, semitones: ) {
        todo!()
    }
    pub fn _reprInternal(&self) -> String {
        todo!()
    }
    pub fn __eq__(&self, other: ) {
        todo!()
    }
    pub fn __hash__(&self) {
        todo!()
    }
    pub fn cents(&self) -> f64 {
        todo!()
    }
    pub fn directed(&self) {
        todo!()
    }
    pub fn undirected(&self) {
        todo!()
    }
    pub fn direction(&self) -> Direction {
        todo!()
    }
    pub fn mod12(&self) {
        todo!()
    }
    pub fn simpleUndirected(&self) {
        todo!()
    }
    pub fn simpleDirected(&self) {
        todo!()
    }
    pub fn intervalClass(&self) -> i32 {
        todo!()
    }
    pub fn isChromaticStep(&self) -> bool {
        todo!()
    }
    pub fn isStep(&self) -> bool {
        todo!()
    }
    pub fn reverse(&self) -> ChromaticInterval {
        todo!()
    }
    pub fn getDiatonic(&self) -> DiatonicInterval {
        todo!()
    }
    pub fn transposePitch(&self, p: ) {
        todo!()
    }
}