use crate::base::music21object::Music21Object;

#[derive(PartialEq)]
pub struct GeneralNote {
    music21object: Music21Object,
    isNote: bool,
    isRest: bool,
    isChord: bool,
    _styleClass: ,
    equalityAttributes: ,

}

impl GeneralNote {
    pub fn new() -> GeneralNote {
        GeneralNote {
            base::music21object: base::Music21Object::new(),
            isNote: todo!(),
            isRest: todo!(),
            isChord: todo!(),
            _styleClass: todo!(),
            equalityAttributes: todo!(),

        }
    }
    
    pub fn new(&self) {
        todo!()
    }
    pub fn __eq__(&self, other: ) {
        todo!()
    }
    pub fn __hash__(&self) {
        todo!()
    }
    pub fn tie(&self) {
        todo!()
    }
    pub fn tie(&self, value: ) {
        todo!()
    }
    pub fn lyric(&self) {
        todo!()
    }
    pub fn lyric(&self, value: ) {
        todo!()
    }
    pub fn addLyric(&self, text: ) {
        todo!()
    }
    pub fn insertLyric(&self, text: ) {
        todo!()
    }
    pub fn fullName(&self) -> String {
        todo!()
    }
    pub fn pitches(&self) {
        todo!()
    }
    pub fn pitches(&self, _value: ) {
        todo!()
    }
    pub fn augmentOrDiminish(&self, scalar: ) {
        todo!()
    }
    pub fn getGrace(&self) {
        todo!()
    }
}