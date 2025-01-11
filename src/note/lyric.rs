use crate::{prebase::protom21object::ProtoM21Object, style::StyleType};

pub(crate) struct Lyric {
    protom21object:ProtoM21Object,
    _style_class:  StyleType ,

}

impl Lyric {
    pub(crate) fn new() -> Lyric {
        Lyric {
            prebase::protom21object: prebase::ProtoM21Object::new(),
            _style_class: todo!(),

        }
    }
    
    pub(crate) fn new(&self, text: String, number: IntegerType) {
        todo!()
    }
    pub(crate) fn _reprInternal(&self) {
        todo!()
    }
    pub(crate) fn isComposite(&self) -> bool {
        todo!()
    }
    pub(crate) fn text(&self) -> String {
        todo!()
    }
    pub(crate) fn text(&self, newText: String) {
        todo!()
    }
    pub(crate) fn syllabic(&self) {
        todo!()
    }
    pub(crate) fn syllabic(&self, newSyllabic: ) {
        todo!()
    }
    pub(crate) fn identifier(&self) {
        todo!()
    }
    pub(crate) fn identifier(&self, value: ) {
        todo!()
    }
    pub(crate) fn rawText(&self) -> String {
        todo!()
    }
    pub(crate) fn rawText(&self, rawTextIn: String) {
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