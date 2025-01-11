use super::intervalbase::IntervalBase;
use crate::{note::note::Note, pitch::pitch::Pitch};

pub(crate) struct Interval {}

impl Interval {
    pub(crate) fn new() -> Interval {
        Interval {}
    }

    // pub(crate) fn _reprInternal(&self) {
    //     todo!()
    // }
    // pub(crate) fn __eq__(&self, other: ) {
    //     todo!()
    // }
    // pub(crate) fn __hash__(&self) {
    //     todo!()
    // }
    // pub(crate) fn generic(&self) -> GenericInterval {
    //     todo!()
    // }
    // pub(crate) fn name(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn niceName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn simpleName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn simpleNiceName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn semiSimpleName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn semiSimpleNiceName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn directedName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn directedNiceName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn directedSimpleName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn directedSimpleNiceName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn semitones(&self) {
    //     todo!()
    // }
    // pub(crate) fn direction(&self) {
    //     todo!()
    // }
    // pub(crate) fn specifier(&self) {
    //     todo!()
    // }
    // pub(crate) fn specificName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn isDiatonicStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isChromaticStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isSkip(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isConsonant(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn complement(&self) -> Interval {
    //     todo!()
    // }
    // pub(crate) fn intervalClass(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn cents(&self) -> FloatType {
    //     todo!()
    // }
    // pub(crate) fn _diatonicIntervalCentShift(&self) -> FloatType {
    //     todo!()
    // }
    // pub(crate) fn transposePitch(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn _diatonicTransposePitch(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn reverse(&self) {
    //     todo!()
    // }
    // pub(crate) fn pitchStart(&self) {
    //     todo!()
    // }
    // pub(crate) fn pitchStart(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn pitchEnd(&self) {
    //     todo!()
    // }
    // pub(crate) fn pitchEnd(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn noteStart(&self) {
    //     todo!()
    // }
    // pub(crate) fn noteStart(&self, n: ) {
    //     todo!()
    // }
    // pub(crate) fn noteEnd(&self) {
    //     todo!()
    // }
    // pub(crate) fn noteEnd(&self, n: ) {
    //     todo!()
    // }
}

impl IntervalBase for Interval {
    fn transpose_note(&self, note1: Note) -> Note {
        let _ = note1;
        todo!()
    }

    fn transpose_pitch(&self, pitch1: Pitch, inplace: bool) -> Pitch {
        let _ = inplace;
        let _ = pitch1;
        todo!()
    }

    fn reverse(&self) {
        todo!()
    }

    fn clear_cache(&self) {
        todo!()
    }
}
