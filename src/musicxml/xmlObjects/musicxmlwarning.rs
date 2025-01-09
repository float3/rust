pub struct MusicXMLWarning {
    userwarning: UserWarning,
}

impl MusicXMLWarning {
    pub fn new() -> MusicXMLWarning {
        MusicXMLWarning {
            userwarning: UserWarning::new(),
        }
    }
}
