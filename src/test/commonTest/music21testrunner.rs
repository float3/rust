pub struct Music21TestRunner {
    unittest::runner::texttestrunner: unittest::runner::TextTestRunner,
}

impl Music21TestRunner {
    pub fn new() -> Music21TestRunner {
        Music21TestRunner {
            unittest::runner::texttestrunner: unittest::runner::TextTestRunner::new(),
        }
    }
    
    pub fn run(&self, test: ) {
        todo!()
    }
}