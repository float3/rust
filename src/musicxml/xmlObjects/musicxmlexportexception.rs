pub struct MusicXMLExportException {
    musicxmlexception: MusicXMLException,
}

impl MusicXMLExportException {
    pub fn new() -> MusicXMLExportException {
        MusicXMLExportException {
            musicxmlexception: MusicXMLException::new(),
        }
    }
}
