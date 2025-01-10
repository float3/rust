use super::intervalbase::IntervalBase;

pub(crate) struct ChromaticInterval {
    intervalbase: IntervalBase,
    semitones: i32,
}

impl ChromaticInterval {
    pub(crate) fn new(semitones: i32) -> ChromaticInterval {
        ChromaticInterval {
            intervalbase: IntervalBase::new(),
            semitones,
        }
    }

    // pub(crate)  fn _reprInternal(&self) -> String {
    //     todo!()
    // }
    // pub(crate)  fn __eq__(&self, other: ) {
    //     todo!()
    // }
    // pub(crate)  fn __hash__(&self) {
    //     todo!()
    // }
    // pub(crate)  fn cents(&self) -> f64 {
    //     todo!()
    // }
    // pub(crate)  fn directed(&self) {
    //     todo!()
    // }
    // pub(crate)  fn undirected(&self) {
    //     todo!()
    // }
    // pub(crate)  fn direction(&self) -> Direction {
    //     todo!()
    // }
    // pub(crate)  fn mod12(&self) {
    //     todo!()
    // }
    // pub(crate)  fn simpleUndirected(&self) {
    //     todo!()
    // }
    // pub(crate)  fn simpleDirected(&self) {
    //     todo!()
    // }
    // pub(crate)  fn intervalClass(&self) -> i32 {
    //     todo!()
    // }
    // pub(crate)  fn isChromaticStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate)  fn isStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate)  fn reverse(&self) -> ChromaticInterval {
    //     todo!()
    // }
    // pub(crate)  fn getDiatonic(&self) -> DiatonicInterval {
    //     todo!()
    // }
    // pub(crate)  fn transposePitch(&self, p: ) {
    //     todo!()
    // }
}
