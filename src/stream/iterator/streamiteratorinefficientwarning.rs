pub struct StreamIteratorInefficientWarning {
    userwarning: UserWarning,
}

impl StreamIteratorInefficientWarning {
    pub fn new() -> StreamIteratorInefficientWarning {
        StreamIteratorInefficientWarning {
            userwarning: UserWarning::new(),
        }
    }
}
