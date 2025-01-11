pub(crate) struct TestCuthbert {
    unittest::testcase: unittest::TestCase,
}

impl TestCuthbert {
    pub(crate) fn new() -> TestCuthbert {
        TestCuthbert {
            unittest::testcase: unittest::TestCase::new(),
        }
    }
    
    pub(crate) fn testCuthbertLocal(&self) {
        todo!()
    }
}