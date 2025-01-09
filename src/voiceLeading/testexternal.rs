pub struct TestExternal {
    unittest::testcase: unittest::TestCase,
}

impl TestExternal {
    pub fn new() -> TestExternal {
        TestExternal {
            unittest::testcase: unittest::TestCase::new(),
        }
    }
    
}