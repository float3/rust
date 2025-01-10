pub(crate)  struct IntervalBase {
    base::music21object: base::Music21Object,
}

impl IntervalBase {
    pub(crate)  fn new() -> IntervalBase {
        IntervalBase {
            base::music21object: base::Music21Object::new(),
        }
    }
    
    pub(crate)  fn transposeNote(&self, note1: ) {
        todo!()
    }
    pub(crate)  fn transposePitch(&self, pitch1: ) {
        todo!()
    }
    pub(crate)  fn reverse(&self) {
        todo!()
    }
    
    pub(crate)  fn clear_cache(&self) {
        todo!()
    }
}