pub struct MusicXMLImportException {
    musicxmlexception: MusicXMLException,
}

impl MusicXMLImportException {
    pub fn new() -> MusicXMLImportException {
        MusicXMLImportException {
            musicxmlexception: MusicXMLException::new(),
        }
    }
}
