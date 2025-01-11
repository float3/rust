pub(crate) struct MusicXMLWarning {
    userwarning: UserWarning,
}

impl MusicXMLWarning {
    pub(crate) fn new() -> MusicXMLWarning {
        MusicXMLWarning {
            userwarning: UserWarning::new(),
        }
    }
}
