pub(crate)  struct StreamIteratorInefficientWarning {
    userwarning: UserWarning,
}

impl StreamIteratorInefficientWarning {
    pub(crate)  fn new() -> StreamIteratorInefficientWarning {
        StreamIteratorInefficientWarning {
            userwarning: UserWarning::new(),
        }
    }
}
