pub(crate)  struct NoteException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl NoteException {
    pub(crate)  fn new() -> NoteException {
        NoteException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}