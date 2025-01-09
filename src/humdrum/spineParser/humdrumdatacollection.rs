pub struct HumdrumDataCollection {
    prebase::protom21object: prebase::ProtoM21Object,
}

impl HumdrumDataCollection {
    pub fn new() -> HumdrumDataCollection {
        HumdrumDataCollection {
            prebase::protom21object: prebase::ProtoM21Object::new(),
        }
    }
    
    pub fn new(&self, dataStream: ) {
        todo!()
    }
    pub fn parse(&self) {
        todo!()
    }
    pub fn parseNonOpus(&self, dataStream: ) {
        todo!()
    }
    pub fn determineIfDataStreamIsOpus(&self, dataStream: ) {
        todo!()
    }
    pub fn parseOpusDataCollections(&self, dataCollections: ) {
        todo!()
    }
    pub fn parseEventListFromDataStream(&self, dataStream: ) {
        todo!()
    }
    pub fn parseProtoSpinesAndEventCollections(&self) {
        todo!()
    }
    pub fn createHumdrumSpines(&self, protoSpines: ) {
        todo!()
    }
    pub fn insertGlobalEvents(&self) {
        todo!()
    }
    pub fn parseMetadata(&self, s: ) {
        todo!()
    }
}