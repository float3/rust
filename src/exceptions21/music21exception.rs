pub struct Music21Exception {
    exception: Exception,
}

impl Music21Exception {
    pub fn new() -> Music21Exception {
        Music21Exception {
            exception: Exception::new(),
        }
    }
    
}