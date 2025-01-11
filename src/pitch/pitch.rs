pub(crate) struct Pitch {
    prebase::protom21object: prebase::ProtoM21Object,
    _DOC_ORDER: ,
    _twelfth_root_of_two: ,

    _SPANISH_DICT: ,
    _FRENCH_DICT: ,
    _transpositionIntervals: ,
}

impl Pitch {
    pub(crate) fn new() -> Pitch {
        Pitch {
            prebase::protom21object: prebase::ProtoM21Object::new(),
            _DOC_ORDER: todo!(),
            _twelfth_root_of_two: todo!(),

            _SPANISH_DICT: todo!(),
            _FRENCH_DICT: todo!(),
            _transpositionIntervals: todo!(),
        }
    }
    
    pub(crate) fn new(&self, name: ) {
        todo!()
    }
    pub(crate) fn _reprInternal(&self) {
        todo!()
    }
    pub(crate) fn __str__(&self) {
        todo!()
    }
    pub(crate) fn __eq__(&self, other: ) {
        todo!()
    }
    pub(crate) fn __deepcopy__(&self, memo: ) {
        todo!()
    }
    pub(crate) fn __hash__(&self) {
        todo!()
    }
    pub(crate) fn __lt__(&self, other: ) -> bool {
        todo!()
    }
    pub(crate) fn __le__(&self, other: ) -> bool {
        todo!()
    }
    pub(crate) fn __gt__(&self, other: ) -> bool {
        todo!()
    }
    pub(crate) fn __ge__(&self, other: ) -> bool {
        todo!()
    }
    pub(crate) fn groups(&self) {
        todo!()
    }
    pub(crate) fn groups(&self, new: ) {
        todo!()
    }
    pub(crate) fn accidental(&self) {
        todo!()
    }
    pub(crate) fn accidental(&self, value: ) {
        todo!()
    }
    pub(crate) fn microtone(&self) -> Microtone {
        todo!()
    }
    pub(crate) fn microtone(&self, value: ) {
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
    pub(crate) fn ps(&self, value: ) {
        todo!()
    }
    pub(crate) fn midi(&self) {
        todo!()
    }
    pub(crate) fn midi(&self, value: ) {
        todo!()
    }
    pub(crate) fn name(&self) -> String {
        todo!()
    }
    pub(crate) fn name(&self, usrStr: String) {
        todo!()
    }
    pub(crate) fn unicodeName(&self) -> String {
        todo!()
    }
    pub(crate) fn nameWithOctave(&self) -> String {
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
    pub(crate) fn set_step(&self, usrStr: ) {
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
    pub(crate) fn octave(&self) {
        todo!()
    }
    pub(crate) fn setOctave(&self, value: ) {
        todo!()
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