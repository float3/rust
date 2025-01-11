pub(crate) struct StreamDeprecationWarning {
    userwarning: UserWarning,
}

impl StreamDeprecationWarning {
    pub(crate) fn new() -> StreamDeprecationWarning {
        StreamDeprecationWarning {
            userwarning: UserWarning::new(),
        }
    }
}
