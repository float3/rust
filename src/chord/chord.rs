use crate::note::note::Note;

use super::chordbase::ChordBase;

#[derive(PartialEq)]
pub struct Chord {
    chordbase: ChordBase,
    isChord: bool,

}

impl Chord {
    pub fn new(&self, notes:  &[Note]) -> Chord {
        todo!()
    }
    
    pub fn __getitem__(&self, key: ) {
        todo!()
    }
    pub fn __setitem__(&self, key: ) {
        todo!()
    }
    pub fn _reprInternal(&self) -> String {
        todo!()
    }
    pub fn formatVectorString(&self, vectorList: ) -> String {
        todo!()
    }
    pub fn _findBass(&self) {
        todo!()
    }
    pub fn _removePitchByRedundantAttribute(&self, attribute: String) {
        todo!()
    }
    pub fn add(&self, notes: ) {
        todo!()
    }
    pub fn annotateIntervals(&self) {
        todo!()
    }
    pub fn annotateIntervals(&self) {
        todo!()
    }
    pub fn annotateIntervals(&self) {
        todo!()
    }
    pub fn annotateIntervals(&self) {
        todo!()
    }
    pub fn areZRelations(&self, other: ) -> bool {
        todo!()
    }
    pub fn bass(&self, newbass: ) {
        todo!()
    }
    pub fn bass(&self, newbass: ) {
        todo!()
    }
    pub fn bass(&self, newbass: ) {
        todo!()
    }
    pub fn canBeDominantV(&self) -> bool {
        todo!()
    }
    pub fn canBeTonic(&self) -> bool {
        todo!()
    }
    pub fn closedPosition(&self) {
        todo!()
    }
    pub fn closedPosition(&self) {
        todo!()
    }
    pub fn closedPosition(&self) {
        todo!()
    }
    pub fn containsSeventh(&self) -> bool {
        todo!()
    }
    pub fn containsTriad(&self) -> bool {
        todo!()
    }
    pub fn _findRoot(&self) {
        todo!()
    }
    pub fn geometricNormalForm(&self) {
        todo!()
    }
    pub fn getChordStep(&self, chordStep: i32) {
        todo!()
    }
    pub fn getColor(&self, pitchTarget: ) {
        todo!()
    }
    pub fn getNotehead(&self, p: ) {
        todo!()
    }
    pub fn getNoteheadFill(&self, p: ) {
        todo!()
    }
    pub fn getStemDirection(&self, p: ) {
        todo!()
    }
    pub fn getTie(&self, p: ) {
        todo!()
    }
    pub fn getVolume(&self, p: ) {
        todo!()
    }
    pub fn getZRelation(&self) {
        todo!()
    }
    pub fn hasAnyEnharmonicSpelledPitches(&self) -> bool {
        todo!()
    }
    pub fn hasAnyRepeatedDiatonicNote(&self) -> bool {
        todo!()
    }
    pub fn hasRepeatedChordStep(&self, chordStep: ) {
        todo!()
    }
    pub fn intervalFromChordStep(&self, chordStep: ) {
        todo!()
    }
    pub fn inversion(&self, newInversion: i32) {
        todo!()
    }
    pub fn inversion(&self, newInversion: ) -> i32 {
        todo!()
    }
    pub fn inversion(&self, newInversion: ) {
        todo!()
    }
    pub fn _setInversion(&self, newInversion: i32, rootPitch: , transposeOnSet: bool) {
        todo!()
    }
    pub fn _findInversion(&self, rootPitch: ) -> i32 {
        todo!()
    }
    pub fn inversionName(&self) {
        todo!()
    }
    pub fn inversionText(&self) -> String {
        todo!()
    }
    pub fn isAugmentedSixth(&self) {
        todo!()
    }
    pub fn isAugmentedTriad(&self) {
        todo!()
    }
    pub fn isConsonant(&self) {
        todo!()
    }
    pub fn isDiminishedSeventh(&self) {
        todo!()
    }
    pub fn isSeventhOfType(&self, intervalArray: ) {
        todo!()
    }
    pub fn isDiminishedTriad(&self) -> bool {
        todo!()
    }
    pub fn isDominantSeventh(&self) -> bool {
        todo!()
    }
    pub fn isFalseDiminishedSeventh(&self) -> bool {
        todo!()
    }
    pub fn isFrenchAugmentedSixth(&self) -> bool {
        todo!()
    }
    pub fn isGermanAugmentedSixth(&self) -> bool {
        todo!()
    }
    pub fn isHalfDiminishedSeventh(&self) -> bool {
        todo!()
    }
    pub fn isIncompleteMajorTriad(&self) -> bool {
        todo!()
    }
    pub fn isIncompleteMinorTriad(&self) -> bool {
        todo!()
    }
    pub fn isItalianAugmentedSixth(&self) -> bool {
        todo!()
    }
    pub fn _isAugmentedSixthHelper(&self, chordTableAddress: ) -> bool {
        todo!()
    }
    pub fn _checkTriadType(&self, chordAddress: ) {
        todo!()
    }
    pub fn isMajorTriad(&self) {
        todo!()
    }
    pub fn isMinorTriad(&self) {
        todo!()
    }
    pub fn isTranspositionallySymmetrical(&self) -> bool {
        todo!()
    }
    pub fn isSeventh(&self) {
        todo!()
    }
    pub fn isNinth(&self) {
        todo!()
    }
    pub fn isSwissAugmentedSixth(&self) {
        todo!()
    }
    pub fn isTriad(&self) -> bool {
        todo!()
    }
    pub fn removeRedundantPitches(&self) {
        todo!()
    }
    pub fn removeRedundantPitchClasses(&self) {
        todo!()
    }
    pub fn removeRedundantPitchNames(&self) {
        todo!()
    }
    pub fn root(&self, newroot: ) {
        todo!()
    }
    pub fn root(&self, newroot: ) {
        todo!()
    }
    pub fn root(&self, newroot: ) {
        todo!()
    }
    pub fn semiClosedPosition(&self) {
        todo!()
    }
    pub fn semiClosedPosition(&self) {
        todo!()
    }
    pub fn semiClosedPosition(&self) {
        todo!()
    }
    pub fn semitonesFromChordStep(&self, chordStep: ) {
        todo!()
    }
    pub fn setColor(&self, value: ) {
        todo!()
    }
    pub fn setNotehead(&self, nh: ) {
        todo!()
    }
    pub fn setNoteheadFill(&self, nh: ) {
        todo!()
    }
    pub fn setStemDirection(&self, stem: ) {
        todo!()
    }
    pub fn setTie(&self, tieObjOrStr: ) {
        todo!()
    }
    pub fn setVolume(&self, vol: ) {
        todo!()
    }
    pub fn simplifyEnharmonics(&self) {
        todo!()
    }
    pub fn sortAscending(&self) {
        todo!()
    }
    pub fn sortChromaticAscending(&self) {
        todo!()
    }
    pub fn sortDiatonicAscending(&self) {
        todo!()
    }
    pub fn sortFrequencyAscending(&self) {
        todo!()
    }
    pub fn transpose(&self, value: ) {
        todo!()
    }
    pub fn chordTablesAddress(&self) {
        todo!()
    }
    pub fn commonName(&self) -> String {
        todo!()
    }
    pub fn duration(&self) -> Duration {
        todo!()
    }
    pub fn duration(&self, durationObj: Duration) {
        todo!()
    }
    pub fn fifth(&self) {
        todo!()
    }
    pub fn forteClass(&self) {
        todo!()
    }
    pub fn forteClassNumber(&self) {
        todo!()
    }
    pub fn forteClassTn(&self) {
        todo!()
    }
    pub fn forteClassTnI(&self) {
        todo!()
    }
    pub fn fullName(&self) {
        todo!()
    }
    pub fn hasZRelation(&self) {
        todo!()
    }
    pub fn intervalVector(&self) {
        todo!()
    }
    pub fn intervalVectorString(&self) {
        todo!()
    }
    pub fn isPrimeFormInversion(&self) {
        todo!()
    }
    pub fn multisetCardinality(&self) {
        todo!()
    }
    pub fn notes(&self) {
        todo!()
    }
    pub fn notes(&self, newNotes: ) {
        todo!()
    }
    pub fn normalOrder(&self) {
        todo!()
    }
    pub fn normalOrderString(&self) {
        todo!()
    }
    pub fn _unorderedPitchClasses(&self) {
        todo!()
    }
    pub fn orderedPitchClasses(&self) {
        todo!()
    }
    pub fn orderedPitchClassesString(&self) -> String {
        todo!()
    }
    pub fn pitchClassCardinality(&self) -> i32 {
        todo!()
    }
    pub fn pitchClasses(&self) {
        todo!()
    }
    pub fn pitchNames(&self) {
        todo!()
    }
    pub fn pitchNames(&self, value: ) {
        todo!()
    }
    pub fn pitchedCommonName(&self) -> String {
        todo!()
    }
    pub fn pitches(&self) {
        todo!()
    }
    pub fn pitches(&self, value: ) {
        todo!()
    }
    pub fn primeForm(&self) {
        todo!()
    }
    pub fn primeFormString(&self) -> String {
        todo!()
    }
    pub fn quality(&self) {
        todo!()
    }
    pub fn scaleDegrees(&self) {
        todo!()
    }
    pub fn seventh(&self) {
        todo!()
    }
    pub fn third(&self) {
        todo!()
    }
}