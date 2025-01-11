pub(crate) struct ABCParenStop {
    abctoken: ABCToken,

}

impl ABCParenStop {
    pub(crate) fn new() -> ABCParenStop {
        ABCParenStop {
            abctoken: ABCToken::new(),

        }
    }
    
}