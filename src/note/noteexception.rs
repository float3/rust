pub struct NoteException {
    exceptions21::music21exception: exceptions21::Music21Exception,
}

impl NoteException {
    pub fn new() -> NoteException {
        NoteException {
            exceptions21::music21exception: exceptions21::Music21Exception::new(),
        }
    }
    
}