use std::rc::Rc;

use crate::note::{note::Note, notrest::NotRest};

#[derive(PartialEq, Clone, Debug)]
pub(crate)  struct ChordBase {
    notes: Vec<Rc<Note>>,
    notrest: NotRest,
    isNote: bool,
    isRest: bool,
}

impl ChordBase {
    pub(crate)  fn new(notes: &[Note]) -> ChordBase {
        ChordBase {
            notes: notes.iter().map(|note| Rc::new((*note).clone())).collect(),
            notrest: NotRest::new(),
            isNote: todo!(),
            isRest: todo!(),
        }
    }
    // pub(crate)  fn __deepcopy__(&self, memo: ) {
    //     todo!()
    // }
    // pub(crate)  fn __iter__(&self) {
    //     todo!()
    // }
    // pub(crate)  fn __len__(&self) {
    //     todo!()
    // }
    // pub(crate)  fn _add_core_or_init(&self, notes: ) {
    //     todo!()
    // }
    pub(crate)  fn add(&self, notes: &[Note]) {
        let _ = notes;
        todo!()
    }
    // pub(crate)  fn remove(&self, removeItem: ) {
    //     todo!()
    // }
    pub(crate)  fn notes(&self) -> &Vec<Rc<Note>> {
        &self.notes
    }
    // pub(crate)  fn tie(&self) {
    //     todo!()
    // }
    // pub(crate)  fn tie(&self, value: ) {
    //     todo!()
    // }
    // pub(crate)  fn volume(&self) {
    //     todo!()
    // }
    // pub(crate)  fn volume(&self, expr: ) {
    //     todo!()
    // }
    // pub(crate)  fn hasComponentVolumes(&self) -> bool {
    //     todo!()
    // }
    // pub(crate)  fn setVolumes(&self, volumes: ) {
    //     todo!()
    // }
}
