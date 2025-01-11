pub(crate) struct TestSlow {
    unittest::testcase: unittest::TestCase,
}

impl TestSlow {
    pub(crate) fn new() -> TestSlow {
        TestSlow {
            unittest::testcase: unittest::TestCase::new(),
        }
    }
    
    pub(crate) fn testMusicXMLConversion(&self) {
        todo!()
    }
}