pub(crate)  struct TimespanTreeException {
    exceptions21::treeexception: exceptions21::TreeException,
}

impl TimespanTreeException {
    pub(crate)  fn new() -> TimespanTreeException {
        TimespanTreeException {
            exceptions21::treeexception: exceptions21::TreeException::new(),
        }
    }
    
}