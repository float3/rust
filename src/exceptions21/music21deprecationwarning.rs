pub(crate)  struct Music21DeprecationWarning {
    userwarning: UserWarning,
}

impl Music21DeprecationWarning {
    pub(crate)  fn new() -> Music21DeprecationWarning {
        Music21DeprecationWarning {
            userwarning: UserWarning::new(),
        }
    }
    
}