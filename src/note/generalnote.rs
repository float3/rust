use crate::{base::music21object::Music21Object, style::StyleType};

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct GeneralNote {
    music21object: Music21Object,
    is_note: bool,
    is_rest: bool,
    is_chord: bool,
    _style_class: StyleType,
    equality_attributes: String,
}

impl GeneralNote {
    pub(crate) fn new() -> GeneralNote {
        GeneralNote {
            music21object: Music21Object::new(None),
            is_note: todo!(),
            is_rest: todo!(),
            is_chord: todo!(),
            _style_class: todo!(),
            equality_attributes: todo!(),
        }
    }

    // pub(crate) fn __eq__(&self, other: ) {
    //     todo!()
    // }
    // pub(crate) fn __hash__(&self) {
    //     todo!()
    // }
    // pub(crate) fn tie(&self) {
    //     todo!()
    // }
    // pub(crate) fn tie(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn lyric(&self) {
    //     todo!()
    // }
    // pub(crate) fn lyric(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn addLyric(&self, text: ) {
    //     todo!()
    // }
    // pub(crate) fn insertLyric(&self, text: ) {
    //     todo!()
    // }
    // pub(crate) fn fullName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn pitches(&self) {
    //     todo!()
    // }
    // pub(crate) fn pitches(&self, _value: ) {
    //     todo!()
    // }
    // pub(crate) fn augmentOrDiminish(&self, scalar: ) {
    //     todo!()
    // }
    // pub(crate) fn getGrace(&self) {
    //     todo!()
    // }
}
