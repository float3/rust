pub(crate)  struct Test {
    unittest::testcase: unittest::TestCase,
}

impl Test {
    pub(crate)  fn new() -> Test {
        Test {
            unittest::testcase: unittest::TestCase::new(),
        }
    }
    
}