pub struct ABCMetadata {
    abctoken: ABCToken,
    __slots__: ,
}

impl ABCMetadata {
    pub fn new() -> ABCMetadata {
        ABCMetadata {
            abctoken: ABCToken::new(),
            __slots__: todo!(),
        }
    }
    
    pub fn new(&self, src: String) {
        todo!()
    }
    pub fn preParse(&self) {
        todo!()
    }
    pub fn parse(&self) {
        todo!()
    }
    pub fn isDefaultNoteLength(&self) -> bool {
        todo!()
    }
    pub fn isReferenceNumber(&self) -> bool {
        todo!()
    }
    pub fn isVersion(&self) -> bool {
        todo!()
    }
    pub fn isMeter(&self) -> bool {
        todo!()
    }
    pub fn isTitle(&self) -> bool {
        todo!()
    }
    pub fn isComposer(&self) -> bool {
        todo!()
    }
    pub fn isOrigin(&self) -> bool {
        todo!()
    }
    pub fn isVoice(&self) -> bool {
        todo!()
    }
    pub fn isKey(&self) -> bool {
        todo!()
    }
    pub fn isTempo(&self) -> bool {
        todo!()
    }
    pub fn getTimeSignatureParameters(&self) {
        todo!()
    }
    pub fn getTimeSignatureObject(&self) {
        todo!()
    }
    pub fn getKeySignatureParameters(&self) {
        todo!()
    }
    pub fn getKeySignatureObject(&self) {
        todo!()
    }
    pub fn getClefObject(&self) {
        todo!()
    }
    pub fn getMetronomeMarkObject(&self) {
        todo!()
    }
    pub fn getDefaultQuarterLength(&self) -> f64 {
        todo!()
    }
}