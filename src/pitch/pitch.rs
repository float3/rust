use std::{cmp::Ordering, fmt::{Display, Formatter, Result}, hash::{Hash, Hasher}};

use crate::{base::groups::Groups, common::types::{Octave, StringOrIntegerOrFLoatOrAccidental, StringOrIntegerOrFloat}, defaults::{FloatType, IntegerType}, interval::stepname::StepName, prebase::protom21object::ProtoM21Object};
use super::{accidental::Accidental, microtone::Microtone};

pub(crate) struct Pitch {
    protom21object: ProtoM21Object,
    octave: Octave,
    microtone: Option<Microtone>,
    groups: Option<Groups>,
    // _DOC_ORDER: ,
    // _twelfth_root_of_two: ,

    // _SPANISH_DICT: ,
    // _FRENCH_DICT: ,
    // _transpositionIntervals: ,
}

impl Pitch {
    pub(crate) fn new(name: StringOrIntegerOrFloat) -> Self {
        todo!()
    }

    pub(crate) fn groups(&self) -> Groups {
        if self.groups == None {
            self.groups = Some(Groups::new());
        }
        self.groups
    }
    pub(crate) fn set_groups(&self, new: Groups ) {
        self.groups = Some(new);
    }
    pub(crate) fn accidental(&self) -> Option<Accidental> {
        todo!()
    }
    pub(crate) fn set_accidental(&self, value: Option<StringOrIntegerOrFLoatOrAccidental>) {
        todo!()
    }
    pub(crate) fn microtone(&self) -> Microtone {
        todo!()
    }
    pub(crate) fn set_microtone(&self, value: ) {
        todo!()
    }
    pub(crate) fn isTwelveTone(&self) -> bool {
        todo!()
    }
    pub(crate) fn getCentShiftFromMidi(&self) -> IntegerType {
        todo!()
    }
    pub(crate) fn alter(&self) -> FloatType {
        todo!()
    }
    pub(crate) fn convertQuarterTonesToMicrotones(&self) {
        todo!()
    }
    pub(crate) fn convertMicrotonesToQuarterTones(&self) {
        todo!()
    }
    pub(crate) fn ps(&self) {
        todo!()
    }
    pub(crate) fn set_ps(&self, value: ) {
        todo!()
    }
    pub(crate) fn midi(&self) {
        todo!()
    }
    pub(crate) fn set_midi(&self, value: ) {
        todo!()
    }
    pub(crate) fn name(&self) -> String {
        todo!()
    }
    pub(crate) fn set_name(&self, usrStr: String) {
        todo!()
    }
    pub(crate) fn unicodeName(&self) -> String {
        todo!()
    }
    pub(crate) fn name_with_octave(&self) -> String {
        todo!()
    }
    pub(crate) fn setNameWithOctave(&self, value: String) {
        todo!()
    }
    pub(crate) fn unicodeNameWithOctave(&self) -> String {
        todo!()
    }
    pub(crate) fn fullName(&self) {
        todo!()
    }
    pub(crate) fn step(&self) {
        todo!()
    }
    pub(crate) fn set_step(&self, usrStr: StepName) {
        todo!()
    }
    pub(crate) fn pitchClass(&self) -> IntegerType {
        todo!()
    }
    pub(crate) fn set_pitchClass(&self, value: ) {
        todo!()
    }
    pub(crate) fn pitchClassString(&self) -> String {
        todo!()
    }
    pub(crate) fn setPitchClassString(&self, v: ) {
        todo!()
    }
    pub(crate) fn octave(&self) -> Octave {
        self.octave
    }
    pub(crate) fn set_octave(&mut self, value: Octave) {
        match value {
            Some(v) => {
                self.octave = Some(v as IntegerType);
            }
            None => {
                self.octave = None;
            }
        }
        self.informClient();
    }
    pub(crate) fn implicitOctave(&self) -> IntegerType {
        todo!()
    }
    pub(crate) fn german(&self) -> String {
        todo!()
    }
    pub(crate) fn italian(&self) -> String {
        todo!()
    }
    pub(crate) fn _getSpanishCardinal(&self) -> String {
        todo!()
    }
    pub(crate) fn spanish(&self) -> String {
        todo!()
    }
    pub(crate) fn french(&self) -> String {
        todo!()
    }
    pub(crate) fn frequency(&self) -> FloatType {
        todo!()
    }
    pub(crate) fn setFrequency(&self, value: ) {
        todo!()
    }
    pub(crate) fn freq440(&self) -> FloatType {
        todo!()
    }
    pub(crate) fn setFreq440(&self, value: ) {
        todo!()
    }
    pub(crate) fn getHarmonic(&self, number: IntegerType) -> Pitch {
        todo!()
    }
    pub(crate) fn harmonicFromFundamental(&self, fundamental: ) -> (IntegerType,FloatType) {
        todo!()
    }
    pub(crate) fn harmonicString(&self, fundamental: ) -> String {
        todo!()
    }
    pub(crate) fn harmonicAndFundamentalFromPitch(&self, target: ) {
        todo!()
    }
    pub(crate) fn harmonicAndFundamentalStringFromPitch(&self, fundamental: ) -> String {
        todo!()
    }
    pub(crate) fn isEnharmonic(&self, other: Pitch) -> bool {
        todo!()
    }
    pub(crate) fn _getEnharmonicHelper(&self, inPlace: , up: bool) {
        todo!()
    }
    pub(crate) fn _getEnharmonicHelper(&self, inPlace: , up: bool) {
        todo!()
    }
    pub(crate) fn _getEnharmonicHelper(&self, inPlace: bool, up: bool) {
        todo!()
    }
    pub(crate) fn getHigherEnharmonic(&self) {
        todo!()
    }
    pub(crate) fn getHigherEnharmonic(&self) {
        todo!()
    }
    pub(crate) fn getHigherEnharmonic(&self) {
        todo!()
    }
    pub(crate) fn getLowerEnharmonic(&self) {
        todo!()
    }
    pub(crate) fn getLowerEnharmonic(&self) {
        todo!()
    }
    pub(crate) fn getLowerEnharmonic(&self) {
        todo!()
    }
    pub(crate) fn simplifyEnharmonic(&self) {
        todo!()
    }
    pub(crate) fn getEnharmonic(&self) {
        todo!()
    }
    pub(crate) fn informClient(&self) {
        todo!()
    }
    pub(crate) fn getAllCommonEnharmonics(&self, alterLimit: IntegerType) {
        todo!()
    }
    pub(crate) fn diatonicNoteNum(&self) -> IntegerType {
        todo!()
    }
    pub(crate) fn diatonicNoteNum(&self, newNum: IntegerType) {
        todo!()
    }
    pub(crate) fn transpose(&self, value: ) {
        todo!()
    }
    pub(crate) fn transpose(&self, value: ) {
        todo!()
    }
    pub(crate) fn transpose(&self, value: ) {
        todo!()
    }
    pub(crate) fn transposeBelowTarget(&self, target: ) {
        todo!()
    }
    pub(crate) fn transposeAboveTarget(&self, target: ) {
        todo!()
    }
    pub(crate) fn _nameInKeySignature(&self, alteredPitches: ) {
        todo!()
    }
    pub(crate) fn _stepInKeySignature(&self, alteredPitches: ) {
        todo!()
    }
    pub(crate) fn updateAccidentalDisplay(&self) {
        todo!()
    }
    pub(crate) fn getStringHarmonic(&self, chordIn: ) {
        todo!()
    }
}

impl Display for Pitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let name = self.name_with_octave(); 
        if let Some(microtone) = self.microtone() {
            // If microtone != 0.0, append cents
            if microtone.cents() != 0.0 {
                write!(f, "{}{}", name, microtone.cents())
            } else {
                write!(f, "{}", name)
            }
        } else {
            write!(f, "{}", name)
        }
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Self) -> bool {
        // Replaces your `__eq__`
        self.octave() == other.octave()
            && self.step() == other.step()
            && self.accidental() == other.accidental()
            && self.microtone() == other.microtone()
    }
}

impl Eq for Pitch {}

impl Hash for Pitch {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.accidental().hash(state);
        self.fundamental().hash(state);
        self.spelling_is_inferred().hash(state);
        self.microtone().hash(state);
        self.octave().hash(state);
        self.step().hash(state);
        std::any::TypeId::of::<Pitch>().hash(state);
    }
}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ps().partial_cmp(&other.ps())
    }
}

impl Ord for Pitch {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ps().partial_cmp(&other.ps()) {
            Some(ordering) => ordering,
            None => Ordering::Equal,
        }
    }
}
