pub(crate)  struct MusicXMLImportException {
    musicxmlexception: MusicXMLException,
}

impl MusicXMLImportException {
    pub(crate)  fn new() -> MusicXMLImportException {
        MusicXMLImportException {
            musicxmlexception: MusicXMLException::new(),
        }
    }
}
