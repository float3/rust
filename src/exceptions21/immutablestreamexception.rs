pub(crate)  struct ImmutableStreamException {
    streamexception: StreamException,
}

impl ImmutableStreamException {
    pub(crate)  fn new() -> ImmutableStreamException {
        ImmutableStreamException {
            streamexception: StreamException::new(),
        }
    }
    
    pub(crate)  fn new(&self, msg: ) {
        todo!()
    }
}