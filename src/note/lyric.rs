use crate::{defaults::IntegerType, prebase::protom21object::ProtoM21Object, style::StyleType};

pub(crate) struct Lyric {
    protom21object:ProtoM21Object,
    _style_class:  StyleType ,

}

impl Lyric {
    pub(crate) fn new(text: String, number: IntegerType) {
        let _ = number;
        let _ = text;
        todo!()
    }
    pub(crate) fn _repr_internal(&self) {
        todo!()
    }
    pub(crate) fn is_composite(&self) -> bool {
        todo!()
    }
    pub(crate) fn text(&self) -> String {
        todo!()
    }
    pub(crate) fn set_text(&self, newText: String) {
        todo!()
    }
    pub(crate) fn syllabic(&self) {
        todo!()
    }
    pub(crate) fn set_syllabic(&self, newSyllabic: ) {
        todo!()
    }
    pub(crate) fn identifier(&self) {
        todo!()
    }
    pub(crate) fn set_identifier(&self, value: ) {
        todo!()
    }
    pub(crate) fn raw_text(&self) -> String {
        todo!()
    }
    pub(crate) fn set_raw_text(&self, rawTextIn: String) {
        todo!()
    }
    pub(crate) fn number(&self) -> IntegerType {
        todo!()
    }
    pub(crate) fn number(&self, value: IntegerType) {
        todo!()
    }
    pub(crate) fn setTextAndSyllabic(&self, rawText: String, applyRaw: bool) {
        todo!()
    }
}