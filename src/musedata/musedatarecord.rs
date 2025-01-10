pub(crate)  struct MuseDataRecord {
    prebase::protom21object: prebase::ProtoM21Object,
}

impl MuseDataRecord {
    pub(crate)  fn new() -> MuseDataRecord {
        MuseDataRecord {
            prebase::protom21object: prebase::ProtoM21Object::new(),
        }
    }
    
    pub(crate)  fn new(&self, src: String, parent: ) {
        todo!()
    }
    pub(crate)  fn isRest(&self) -> bool {
        todo!()
    }
    pub(crate)  fn isTied(&self) {
        todo!()
    }
    pub(crate)  fn isNote(&self) {
        todo!()
    }
    pub(crate)  fn isChord(&self) {
        todo!()
    }
    pub(crate)  fn isCueOrGrace(&self) {
        todo!()
    }
    pub(crate)  fn isBack(&self) {
        todo!()
    }
    pub(crate)  fn _getPitchParameters(&self) {
        todo!()
    }
    pub(crate)  fn _getAccidentalObject(&self) {
        todo!()
    }
    pub(crate)  fn getPitchObject(&self) {
        todo!()
    }
    pub(crate)  fn getQuarterLength(&self, divisionsPerQuarterNote: ) {
        todo!()
    }
    pub(crate)  fn getDots(&self) {
        todo!()
    }
    pub(crate)  fn getLyrics(&self) {
        todo!()
    }
    pub(crate)  fn getBeams(&self) {
        todo!()
    }
    pub(crate)  fn _getAdditionalNotations(&self) {
        todo!()
    }
    pub(crate)  fn getArticulationObjects(&self) {
        todo!()
    }
    pub(crate)  fn getExpressionObjects(&self) {
        todo!()
    }
    pub(crate)  fn getDynamicObjects(&self) {
        todo!()
    }
    pub(crate)  fn hasCautionaryAccidental(&self) {
        todo!()
    }
}