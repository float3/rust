pub struct TestSlow {
    unittest::testcase: unittest::TestCase,
}

impl TestSlow {
    pub fn new() -> TestSlow {
        TestSlow {
            unittest::testcase: unittest::TestCase::new(),
        }
    }
    
    pub fn testMusicXMLConversion(&self) {
        todo!()
    }
}