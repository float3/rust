use super::intervalbase::IntervalBase;
use crate::{note::note::Note, pitch::pitch::Pitch};

pub(crate) struct ChromaticInterval {
    semitones: IntegerType,
}

impl ChromaticInterval {
    pub(crate) fn new(semitones: IntegerType) -> ChromaticInterval {
        ChromaticInterval { semitones }
    }

    // pub(crate) fn _reprInternal(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn __eq__(&self, other: ) {
    //     todo!()
    // }
    // pub(crate) fn __hash__(&self) {
    //     todo!()
    // }
    // pub(crate) fn cents(&self) -> FloatType {
    //     todo!()
    // }
    // pub(crate) fn directed(&self) {
    //     todo!()
    // }
    // pub(crate) fn undirected(&self) {
    //     todo!()
    // }
    // pub(crate) fn direction(&self) -> Direction {
    //     todo!()
    // }
    // pub(crate) fn mod12(&self) {
    //     todo!()
    // }
    // pub(crate) fn simpleUndirected(&self) {
    //     todo!()
    // }
    // pub(crate) fn simpleDirected(&self) {
    //     todo!()
    // }
    // pub(crate) fn intervalClass(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn isChromaticStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn reverse(&self) -> ChromaticInterval {
    //     todo!()
    // }
    // pub(crate) fn getDiatonic(&self) -> DiatonicInterval {
    //     todo!()
    // }
    // pub(crate) fn transposePitch(&self, p: ) {
    //     todo!()
    // }
}

impl IntervalBase for ChromaticInterval {
    fn transpose_note(&self, note1: Note) -> Note {
        todo!()
    }

    fn transpose_pitch(&self, pitch1: Pitch, inplace: bool) -> Pitch {
        todo!()
    }

    fn reverse(&self) {
        todo!()
    }

    fn clear_cache(&self) {
        todo!()
    }
}
