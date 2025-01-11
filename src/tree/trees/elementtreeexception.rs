pub(crate) struct ElementTreeException {
    exceptions21::treeexception: exceptions21::TreeException,
}

impl ElementTreeException {
    pub(crate) fn new() -> ElementTreeException {
        ElementTreeException {
            exceptions21::treeexception: exceptions21::TreeException::new(),
        }
    }
    
}