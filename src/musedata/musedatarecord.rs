pub struct MuseDataRecord {
    prebase::protom21object: prebase::ProtoM21Object,
}

impl MuseDataRecord {
    pub fn new() -> MuseDataRecord {
        MuseDataRecord {
            prebase::protom21object: prebase::ProtoM21Object::new(),
        }
    }
    
    pub fn new(&self, src: String, parent: ) {
        todo!()
    }
    pub fn isRest(&self) -> bool {
        todo!()
    }
    pub fn isTied(&self) {
        todo!()
    }
    pub fn isNote(&self) {
        todo!()
    }
    pub fn isChord(&self) {
        todo!()
    }
    pub fn isCueOrGrace(&self) {
        todo!()
    }
    pub fn isBack(&self) {
        todo!()
    }
    pub fn _getPitchParameters(&self) {
        todo!()
    }
    pub fn _getAccidentalObject(&self) {
        todo!()
    }
    pub fn getPitchObject(&self) {
        todo!()
    }
    pub fn getQuarterLength(&self, divisionsPerQuarterNote: ) {
        todo!()
    }
    pub fn getDots(&self) {
        todo!()
    }
    pub fn getLyrics(&self) {
        todo!()
    }
    pub fn getBeams(&self) {
        todo!()
    }
    pub fn _getAdditionalNotations(&self) {
        todo!()
    }
    pub fn getArticulationObjects(&self) {
        todo!()
    }
    pub fn getExpressionObjects(&self) {
        todo!()
    }
    pub fn getDynamicObjects(&self) {
        todo!()
    }
    pub fn hasCautionaryAccidental(&self) {
        todo!()
    }
}