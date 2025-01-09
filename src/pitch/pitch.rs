pub struct Pitch {
    prebase::protom21object: prebase::ProtoM21Object,
    _DOC_ORDER: ,
    _twelfth_root_of_two: ,

    _SPANISH_DICT: ,
    _FRENCH_DICT: ,
    _transpositionIntervals: ,
}

impl Pitch {
    pub fn new() -> Pitch {
        Pitch {
            prebase::protom21object: prebase::ProtoM21Object::new(),
            _DOC_ORDER: todo!(),
            _twelfth_root_of_two: todo!(),
            _DOC_ATTR: todo!(),
            _SPANISH_DICT: todo!(),
            _FRENCH_DICT: todo!(),
            _transpositionIntervals: todo!(),
        }
    }
    
    pub fn new(&self, name: ) {
        todo!()
    }
    pub fn _reprInternal(&self) {
        todo!()
    }
    pub fn __str__(&self) {
        todo!()
    }
    pub fn __eq__(&self, other: ) {
        todo!()
    }
    pub fn __deepcopy__(&self, memo: ) {
        todo!()
    }
    pub fn __hash__(&self) {
        todo!()
    }
    pub fn __lt__(&self, other: ) -> bool {
        todo!()
    }
    pub fn __le__(&self, other: ) -> bool {
        todo!()
    }
    pub fn __gt__(&self, other: ) -> bool {
        todo!()
    }
    pub fn __ge__(&self, other: ) -> bool {
        todo!()
    }
    pub fn groups(&self) {
        todo!()
    }
    pub fn groups(&self, new: ) {
        todo!()
    }
    pub fn accidental(&self) {
        todo!()
    }
    pub fn accidental(&self, value: ) {
        todo!()
    }
    pub fn microtone(&self) -> Microtone {
        todo!()
    }
    pub fn microtone(&self, value: ) {
        todo!()
    }
    pub fn isTwelveTone(&self) -> bool {
        todo!()
    }
    pub fn getCentShiftFromMidi(&self) -> i32 {
        todo!()
    }
    pub fn alter(&self) -> f64 {
        todo!()
    }
    pub fn convertQuarterTonesToMicrotones(&self) {
        todo!()
    }
    pub fn convertMicrotonesToQuarterTones(&self) {
        todo!()
    }
    pub fn ps(&self) {
        todo!()
    }
    pub fn ps(&self, value: ) {
        todo!()
    }
    pub fn midi(&self) {
        todo!()
    }
    pub fn midi(&self, value: ) {
        todo!()
    }
    pub fn name(&self) -> String {
        todo!()
    }
    pub fn name(&self, usrStr: String) {
        todo!()
    }
    pub fn unicodeName(&self) -> String {
        todo!()
    }
    pub fn nameWithOctave(&self) -> String {
        todo!()
    }
    pub fn nameWithOctave(&self, value: String) {
        todo!()
    }
    pub fn unicodeNameWithOctave(&self) -> String {
        todo!()
    }
    pub fn fullName(&self) {
        todo!()
    }
    pub fn step(&self) {
        todo!()
    }
    pub fn step(&self, usrStr: ) {
        todo!()
    }
    pub fn pitchClass(&self) -> i32 {
        todo!()
    }
    pub fn pitchClass(&self, value: ) {
        todo!()
    }
    pub fn pitchClassString(&self) -> String {
        todo!()
    }
    pub fn pitchClassString(&self, v: ) {
        todo!()
    }
    pub fn octave(&self) {
        todo!()
    }
    pub fn octave(&self, value: ) {
        todo!()
    }
    pub fn implicitOctave(&self) -> i32 {
        todo!()
    }
    pub fn german(&self) -> String {
        todo!()
    }
    pub fn italian(&self) -> String {
        todo!()
    }
    pub fn _getSpanishCardinal(&self) -> String {
        todo!()
    }
    pub fn spanish(&self) -> String {
        todo!()
    }
    pub fn french(&self) -> String {
        todo!()
    }
    pub fn frequency(&self) -> f64 {
        todo!()
    }
    pub fn frequency(&self, value: ) {
        todo!()
    }
    pub fn freq440(&self) -> f64 {
        todo!()
    }
    pub fn freq440(&self, value: ) {
        todo!()
    }
    pub fn getHarmonic(&self, number: i32) -> Pitch {
        todo!()
    }
    pub fn harmonicFromFundamental(&self, fundamental: ) {
        todo!()
    }
    pub fn harmonicString(&self, fundamental: ) -> String {
        todo!()
    }
    pub fn harmonicAndFundamentalFromPitch(&self, target: ) {
        todo!()
    }
    pub fn harmonicAndFundamentalStringFromPitch(&self, fundamental: ) -> String {
        todo!()
    }
    pub fn isEnharmonic(&self, other: Pitch) -> bool {
        todo!()
    }
    pub fn _getEnharmonicHelper(&self, inPlace: , up: bool) {
        todo!()
    }
    pub fn _getEnharmonicHelper(&self, inPlace: , up: bool) {
        todo!()
    }
    pub fn _getEnharmonicHelper(&self, inPlace: bool, up: bool) {
        todo!()
    }
    pub fn getHigherEnharmonic(&self) {
        todo!()
    }
    pub fn getHigherEnharmonic(&self) {
        todo!()
    }
    pub fn getHigherEnharmonic(&self) {
        todo!()
    }
    pub fn getLowerEnharmonic(&self) {
        todo!()
    }
    pub fn getLowerEnharmonic(&self) {
        todo!()
    }
    pub fn getLowerEnharmonic(&self) {
        todo!()
    }
    pub fn simplifyEnharmonic(&self) {
        todo!()
    }
    pub fn getEnharmonic(&self) {
        todo!()
    }
    pub fn informClient(&self) {
        todo!()
    }
    pub fn getAllCommonEnharmonics(&self, alterLimit: i32) {
        todo!()
    }
    pub fn diatonicNoteNum(&self) -> i32 {
        todo!()
    }
    pub fn diatonicNoteNum(&self, newNum: i32) {
        todo!()
    }
    pub fn transpose(&self, value: ) {
        todo!()
    }
    pub fn transpose(&self, value: ) {
        todo!()
    }
    pub fn transpose(&self, value: ) {
        todo!()
    }
    pub fn transposeBelowTarget(&self, target: ) {
        todo!()
    }
    pub fn transposeAboveTarget(&self, target: ) {
        todo!()
    }
    pub fn _nameInKeySignature(&self, alteredPitches: ) {
        todo!()
    }
    pub fn _stepInKeySignature(&self, alteredPitches: ) {
        todo!()
    }
    pub fn updateAccidentalDisplay(&self) {
        todo!()
    }
    pub fn getStringHarmonic(&self, chordIn: ) {
        todo!()
    }
}