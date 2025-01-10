pub(crate)  struct MusicXMLExportException {
    musicxmlexception: MusicXMLException,
}

impl MusicXMLExportException {
    pub(crate)  fn new() -> MusicXMLExportException {
        MusicXMLExportException {
            musicxmlexception: MusicXMLException::new(),
        }
    }
}
