pub(crate) struct HumdrumDataCollection {
    protom21object: ProtoM21Object,
}

impl HumdrumDataCollection {
    pub(crate) fn new() -> HumdrumDataCollection {
        HumdrumDataCollection {
            protom21object: ProtoM21Object::new(),
        }
    }
    
    pub(crate) fn new(dataStream: ) {
        todo!()
    }
    pub(crate) fn parse(&self) {
        todo!()
    }
    pub(crate) fn parseNonOpus(&self, dataStream: ) {
        todo!()
    }
    pub(crate) fn determineIfDataStreamIsOpus(&self, dataStream: ) {
        todo!()
    }
    pub(crate) fn parseOpusDataCollections(&self, dataCollections: ) {
        todo!()
    }
    pub(crate) fn parseEventListFromDataStream(&self, dataStream: ) {
        todo!()
    }
    pub(crate) fn parseProtoSpinesAndEventCollections(&self) {
        todo!()
    }
    pub(crate) fn createHumdrumSpines(&self, protoSpines: ) {
        todo!()
    }
    pub(crate) fn insertGlobalEvents(&self) {
        todo!()
    }
    pub(crate) fn parseMetadata(&self, s: ) {
        todo!()
    }
}