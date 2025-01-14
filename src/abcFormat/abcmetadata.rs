pub(crate) struct ABCMetadata {
    abctoken: ABCToken,
}

impl ABCMetadata {
    pub(crate) fn new() -> ABCMetadata {
        ABCMetadata {
            abctoken: ABCToken::new(),
        }
    }

    pub(crate) fn new(src: String) {
        todo!()
    }
    pub(crate) fn preParse(&self) {
        todo!()
    }
    pub(crate) fn parse(&self) {
        todo!()
    }
    pub(crate) fn isDefaultNoteLength(&self) -> bool {
        todo!()
    }
    pub(crate) fn isReferenceNumber(&self) -> bool {
        todo!()
    }
    pub(crate) fn isVersion(&self) -> bool {
        todo!()
    }
    pub(crate) fn isMeter(&self) -> bool {
        todo!()
    }
    pub(crate) fn isTitle(&self) -> bool {
        todo!()
    }
    pub(crate) fn isComposer(&self) -> bool {
        todo!()
    }
    pub(crate) fn isOrigin(&self) -> bool {
        todo!()
    }
    pub(crate) fn isVoice(&self) -> bool {
        todo!()
    }
    pub(crate) fn isKey(&self) -> bool {
        todo!()
    }
    pub(crate) fn isTempo(&self) -> bool {
        todo!()
    }
    pub(crate) fn getTimeSignatureParameters(&self) {
        todo!()
    }
    pub(crate) fn getTimeSignatureObject(&self) {
        todo!()
    }
    pub(crate) fn getKeySignatureParameters(&self) {
        todo!()
    }
    pub(crate) fn getKeySignatureObject(&self) {
        todo!()
    }
    pub(crate) fn getClefObject(&self) {
        todo!()
    }
    pub(crate) fn getMetronomeMarkObject(&self) {
        todo!()
    }
    pub(crate) fn getDefaultQuarterLength(&self) -> FloatType {
        todo!()
    }
}
