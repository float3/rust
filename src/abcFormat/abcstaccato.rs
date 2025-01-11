pub(crate) struct ABCStaccato {
    abctoken: ABCToken,
}

impl ABCStaccato {
    pub(crate) fn new() -> ABCStaccato {
        ABCStaccato {
            abctoken: ABCToken::new(),
        }
    }
}
