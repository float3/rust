pub struct TestCuthbert {
    unittest::testcase: unittest::TestCase,
}

impl TestCuthbert {
    pub fn new() -> TestCuthbert {
        TestCuthbert {
            unittest::testcase: unittest::TestCase::new(),
        }
    }
    
    pub fn testCuthbertLocal(&self) {
        todo!()
    }
}