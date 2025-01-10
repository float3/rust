use crate::style::StyleType;


#[derive(PartialEq, Clone, Debug)]
pub(crate)  struct GeneralNote {
    music21object: Music21Object,
    isNote: bool,
    isRest: bool,
    isChord: bool,
    _styleClass: StyleType ,
    equalityAttributes: String,

}

impl GeneralNote {
    pub(crate)  fn new() -> GeneralNote {
        GeneralNote {
            music21object:Music21Object::new(),
            isNote: todo!(),
            isRest: todo!(),
            isChord: todo!(),
            _styleClass: todo!(),
            equalityAttributes: todo!(),

        }
    }
    
    pub(crate)  fn new(&self) {
        todo!()
    }
    pub(crate)  fn __eq__(&self, other: ) {
        todo!()
    }
    pub(crate)  fn __hash__(&self) {
        todo!()
    }
    pub(crate)  fn tie(&self) {
        todo!()
    }
    pub(crate)  fn tie(&self, value: ) {
        todo!()
    }
    pub(crate)  fn lyric(&self) {
        todo!()
    }
    pub(crate)  fn lyric(&self, value: ) {
        todo!()
    }
    pub(crate)  fn addLyric(&self, text: ) {
        todo!()
    }
    pub(crate)  fn insertLyric(&self, text: ) {
        todo!()
    }
    pub(crate)  fn fullName(&self) -> String {
        todo!()
    }
    pub(crate)  fn pitches(&self) {
        todo!()
    }
    pub(crate)  fn pitches(&self, _value: ) {
        todo!()
    }
    pub(crate)  fn augmentOrDiminish(&self, scalar: ) {
        todo!()
    }
    pub(crate)  fn getGrace(&self) {
        todo!()
    }
}