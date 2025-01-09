pub struct StreamDeprecationWarning {
    userwarning: UserWarning,
}

impl StreamDeprecationWarning {
    pub fn new() -> StreamDeprecationWarning {
        StreamDeprecationWarning {
            userwarning: UserWarning::new(),
        }
    }
}
