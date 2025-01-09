pub struct ImmutableStreamException {
    streamexception: StreamException,
}

impl ImmutableStreamException {
    pub fn new() -> ImmutableStreamException {
        ImmutableStreamException {
            streamexception: StreamException::new(),
        }
    }
    
    pub fn new(&self, msg: ) {
        todo!()
    }
}