use crate::note::note::Note;

use super::chordbase::ChordBase;

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Chord {
    chordbase: ChordBase,
    is_chord: bool,
}

impl Chord {
    pub(crate) fn new(notes: &[Note]) -> Chord {
        Chord {
            chordbase: ChordBase::new(notes),
            is_chord: todo!(),
        }
    }

    // pub(crate) fn __eq__(&self, other: ) {
    //     todo!()
    // }
    // pub(crate) fn __hash__(&self) {
    //     todo!()
    // }
    // pub(crate) fn __getitem__(&self, key: ) {
    //     todo!()
    // }
    // pub(crate) fn __setitem__(&self, key: ) {
    //     todo!()
    // }
    // pub(crate) fn _reprInternal(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn formatVectorString(&self, vectorList: ) -> String {
    //     todo!()
    // }
    pub(crate) fn _find_bass(&self) {
        todo!()
    }
    // pub(crate) fn _removePitchByRedundantAttribute(&self, attribute: String) {
    //     todo!()
    // }
    pub(crate) fn add(&self, notes: &[Note]) {
        let _ = notes;
        todo!()
    }
    pub(crate) fn annotate_intervals(&self) {
        todo!()
    }
    // pub(crate) fn annotateIntervals(&self) {
    //     todo!()
    // }
    // pub(crate) fn annotateIntervals(&self) {
    //     todo!()
    // }
    // pub(crate) fn annotateIntervals(&self) {
    //     todo!()
    // }
    // pub(crate) fn areZRelations(&self, other: ) -> bool {
    //     todo!()
    // }
    // pub(crate) fn bass(&self, newbass: ) {
    //     todo!()
    // }
    // pub(crate) fn bass(&self, newbass: ) {
    //     todo!()
    // }
    // pub(crate) fn bass(&self, newbass: ) {
    //     todo!()
    // }
    // pub(crate) fn canBeDominantV(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn canBeTonic(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn closedPosition(&self) {
    //     todo!()
    // }
    // pub(crate) fn closedPosition(&self) {
    //     todo!()
    // }
    // pub(crate) fn closedPosition(&self) {
    //     todo!()
    // }
    // pub(crate) fn containsSeventh(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn containsTriad(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn _findRoot(&self) {
    //     todo!()
    // }
    // pub(crate) fn geometricNormalForm(&self) {
    //     todo!()
    // }
    // pub(crate) fn getChordStep(&self, chordStep: IntegerType) {
    //     todo!()
    // }
    // pub(crate) fn getColor(&self, pitchTarget: ) {
    //     todo!()
    // }
    // pub(crate) fn getNotehead(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn getNoteheadFill(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn getStemDirection(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn getTie(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn getVolume(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn getZRelation(&self) {
    //     todo!()
    // }
    // pub(crate) fn hasAnyEnharmonicSpelledPitches(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn hasAnyRepeatedDiatonicNote(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn hasRepeatedChordStep(&self, chordStep: ) {
    //     todo!()
    // }
    // pub(crate) fn intervalFromChordStep(&self, chordStep: ) {
    //     todo!()
    // }
    // pub(crate) fn inversion(&self, newInversion: IntegerType) {
    //     todo!()
    // }
    // pub(crate) fn inversion(&self, newInversion: ) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn inversion(&self, newInversion: ) {
    //     todo!()
    // }
    // pub(crate) fn _setInversion(&self, newInversion: IntegerType, rootPitch: , transposeOnSet: bool) {
    //     todo!()
    // }
    // pub(crate) fn _findInversion(&self, rootPitch: ) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn inversionName(&self) {
    //     todo!()
    // }
    // pub(crate) fn inversionText(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn isAugmentedSixth(&self) {
    //     todo!()
    // }
    // pub(crate) fn isAugmentedTriad(&self) {
    //     todo!()
    // }
    // pub(crate) fn isConsonant(&self) {
    //     todo!()
    // }
    // pub(crate) fn isDiminishedSeventh(&self) {
    //     todo!()
    // }
    // pub(crate) fn isSeventhOfType(&self, intervalArray: ) {
    //     todo!()
    // }
    // pub(crate) fn isDiminishedTriad(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isDominantSeventh(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isFalseDiminishedSeventh(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isFrenchAugmentedSixth(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isGermanAugmentedSixth(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isHalfDiminishedSeventh(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isIncompleteMajorTriad(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isIncompleteMinorTriad(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isItalianAugmentedSixth(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn _isAugmentedSixthHelper(&self, chordTableAddress: ) -> bool {
    //     todo!()
    // }
    // pub(crate) fn _checkTriadType(&self, chordAddress: ) {
    //     todo!()
    // }
    // pub(crate) fn isMajorTriad(&self) {
    //     todo!()
    // }
    // pub(crate) fn isMinorTriad(&self) {
    //     todo!()
    // }
    // pub(crate) fn isTranspositionallySymmetrical(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isSeventh(&self) {
    //     todo!()
    // }
    // pub(crate) fn isNinth(&self) {
    //     todo!()
    // }
    // pub(crate) fn isSwissAugmentedSixth(&self) {
    //     todo!()
    // }
    // pub(crate) fn isTriad(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn removeRedundantPitches(&self) {
    //     todo!()
    // }
    // pub(crate) fn removeRedundantPitchClasses(&self) {
    //     todo!()
    // }
    // pub(crate) fn removeRedundantPitchNames(&self) {
    //     todo!()
    // }
    // pub(crate) fn root(&self, newroot: ) {
    //     todo!()
    // }
    // pub(crate) fn root(&self, newroot: ) {
    //     todo!()
    // }
    // pub(crate) fn root(&self, newroot: ) {
    //     todo!()
    // }
    // pub(crate) fn semiClosedPosition(&self) {
    //     todo!()
    // }
    // pub(crate) fn semiClosedPosition(&self) {
    //     todo!()
    // }
    // pub(crate) fn semiClosedPosition(&self) {
    //     todo!()
    // }
    // pub(crate) fn semitonesFromChordStep(&self, chordStep: ) {
    //     todo!()
    // }
    // pub(crate) fn setColor(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn setNotehead(&self, nh: ) {
    //     todo!()
    // }
    // pub(crate) fn setNoteheadFill(&self, nh: ) {
    //     todo!()
    // }
    // pub(crate) fn setStemDirection(&self, stem: ) {
    //     todo!()
    // }
    // pub(crate) fn setTie(&self, tieObjOrStr: ) {
    //     todo!()
    // }
    // pub(crate) fn setVolume(&self, vol: ) {
    //     todo!()
    // }
    // pub(crate) fn simplifyEnharmonics(&self) {
    //     todo!()
    // }
    // pub(crate) fn sortAscending(&self) {
    //     todo!()
    // }
    // pub(crate) fn sortChromaticAscending(&self) {
    //     todo!()
    // }
    // pub(crate) fn sortDiatonicAscending(&self) {
    //     todo!()
    // }
    // pub(crate) fn sortFrequencyAscending(&self) {
    //     todo!()
    // }
    // pub(crate) fn transpose(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn chordTablesAddress(&self) {
    //     todo!()
    // }
    // pub(crate) fn commonName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn duration(&self) -> Duration {
    //     todo!()
    // }
    // pub(crate) fn duration(&self, durationObj: Duration) {
    //     todo!()
    // }
    // pub(crate) fn fifth(&self) {
    //     todo!()
    // }
    // pub(crate) fn forteClass(&self) {
    //     todo!()
    // }
    // pub(crate) fn forteClassNumber(&self) {
    //     todo!()
    // }
    // pub(crate) fn forteClassTn(&self) {
    //     todo!()
    // }
    // pub(crate) fn forteClassTnI(&self) {
    //     todo!()
    // }
    // pub(crate) fn fullName(&self) {
    //     todo!()
    // }
    // pub(crate) fn hasZRelation(&self) {
    //     todo!()
    // }
    // pub(crate) fn intervalVector(&self) {
    //     todo!()
    // }
    // pub(crate) fn intervalVectorString(&self) {
    //     todo!()
    // }
    // pub(crate) fn isPrimeFormInversion(&self) {
    //     todo!()
    // }
    // pub(crate) fn multisetCardinality(&self) {
    //     todo!()
    // }
    // pub(crate) fn notes(&self) {
    //     todo!()
    // }
    // pub(crate) fn notes(&self, newNotes: ) {
    //     todo!()
    // }
    // pub(crate) fn normalOrder(&self) {
    //     todo!()
    // }
    // pub(crate) fn normalOrderString(&self) {
    //     todo!()
    // }
    // pub(crate) fn _unorderedPitchClasses(&self) {
    //     todo!()
    // }
    // pub(crate) fn orderedPitchClasses(&self) {
    //     todo!()
    // }
    // pub(crate) fn orderedPitchClassesString(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn pitchClassCardinality(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn pitchClasses(&self) {
    //     todo!()
    // }
    // pub(crate) fn pitchNames(&self) {
    //     todo!()
    // }
    // pub(crate) fn pitchNames(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn pitchedCommonName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn pitches(&self) {
    //     todo!()
    // }
    // pub(crate) fn pitches(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn primeForm(&self) {
    //     todo!()
    // }
    // pub(crate) fn primeFormString(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn quality(&self) {
    //     todo!()
    // }
    // pub(crate) fn scaleDegrees(&self) {
    //     todo!()
    // }
    // pub(crate) fn seventh(&self) {
    //     todo!()
    // }
    // pub(crate) fn third(&self) {
    //     todo!()
    // }
}
