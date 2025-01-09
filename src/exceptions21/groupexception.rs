pub struct GroupException {
    music21exception: Music21Exception,
}

impl GroupException {
    pub fn new() -> GroupException {
        GroupException {
            music21exception: Music21Exception::new(),
        }
    }
}
