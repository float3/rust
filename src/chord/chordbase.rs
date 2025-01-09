use std::rc::Rc;

use crate::note::{note::Note, notrest::NotRest};

#[derive(PartialEq)]
pub struct ChordBase {
    notes: Vec<Rc<Note>>,
    notrest: NotRest,
    isNote: bool,
    isRest: bool,

}

impl ChordBase {
    pub fn new(&self, notes: &[Note]) -> ChordBase {
        ChordBase {
            notes: notes.iter().map(|note| Rc::new(*note)).collect(),
            notrest: NotRest::new(),
            isNote: todo!(),
            isRest: todo!(),

        }
    }
    pub fn __deepcopy__(&self, memo: ) {
        todo!()
    }
    pub fn __iter__(&self) {
        todo!()
    }
    pub fn __len__(&self) {
        todo!()
    }
    pub fn _add_core_or_init(&self, notes: ) {
        todo!()
    }
    pub fn add(&self, notes: ) {
        todo!()
    }
    pub fn remove(&self, removeItem: ) {
        todo!()
    }
    pub fn notes(&self) {
        todo!()
    }
    pub fn tie(&self) {
        todo!()
    }
    pub fn tie(&self, value: ) {
        todo!()
    }
    pub fn volume(&self) {
        todo!()
    }
    pub fn volume(&self, expr: ) {
        todo!()
    }
    pub fn hasComponentVolumes(&self) -> bool {
        todo!()
    }
    pub fn setVolumes(&self, volumes: ) {
        todo!()
    }
}