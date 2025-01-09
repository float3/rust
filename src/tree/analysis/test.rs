pub struct Test {
    unittest::testcase: unittest::TestCase,
}

impl Test {
    pub fn new() -> Test {
        Test {
            unittest::testcase: unittest::TestCase::new(),
        }
    }
    
}