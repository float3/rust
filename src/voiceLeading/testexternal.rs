pub(crate)  struct TestExternal {
    unittest::testcase: unittest::TestCase,
}

impl TestExternal {
    pub(crate)  fn new() -> TestExternal {
        TestExternal {
            unittest::testcase: unittest::TestCase::new(),
        }
    }
    
}