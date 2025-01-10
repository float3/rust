use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub(crate)  struct Music21Exception {
    error_message: String,
}

impl Music21Exception {
    pub(crate)  fn new(error_message: String) -> Music21Exception {
        Music21Exception { error_message }
    }
    pub(crate)  fn error_message(&self) -> String {
        self.error_message.clone()
    }
}

impl Display for Music21Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl Error for Music21Exception {}
