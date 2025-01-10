pub(crate)  struct Music21TestRunner {
    unittest::runner::texttestrunner: unittest::runner::TextTestRunner,
}

impl Music21TestRunner {
    pub(crate)  fn new() -> Music21TestRunner {
        Music21TestRunner {
            unittest::runner::texttestrunner: unittest::runner::TextTestRunner::new(),
        }
    }
    
    pub(crate)  fn run(&self, test: ) {
        todo!()
    }
}