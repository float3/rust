pub(crate) struct MidiEvent {
    prebase::protom21object: prebase::ProtoM21Object,
    pitch: ,
    velocity: ,
    data: ,
}

impl MidiEvent {
    pub(crate) fn new() -> MidiEvent {
        MidiEvent {
            prebase::protom21object: prebase::ProtoM21Object::new(),
            pitch: todo!(),
            velocity: todo!(),
            data: todo!(),
        }
    }
    
    pub(crate) fn new(&self, track: ) {
        todo!()
    }
    pub(crate) fn sortOrder(&self) -> IntegerType {
        todo!()
    }
    pub(crate) fn _reprInternal(&self) {
        todo!()
    }
    pub(crate) fn _setPitch(&self, value: ) {
        todo!()
    }
    pub(crate) fn _getPitch(&self) {
        todo!()
    }
    pub(crate) fn _setVelocity(&self, value: ) {
        todo!()
    }
    pub(crate) fn _getVelocity(&self) {
        todo!()
    }
    pub(crate) fn _setData(&self, value: ) {
        todo!()
    }
    pub(crate) fn _getData(&self) {
        todo!()
    }
    pub(crate) fn setPitchBend(&self, cents: ) {
        todo!()
    }
    pub(crate) fn parseChannelVoiceMessage(&self, midiBytes: ) {
        todo!()
    }
    pub(crate) fn read(&self, midiBytes: ) {
        todo!()
    }
    pub(crate) fn getBytes(&self) {
        todo!()
    }
    pub(crate) fn isNoteOn(&self) {
        todo!()
    }
    pub(crate) fn isNoteOff(&self) {
        todo!()
    }
    pub(crate) fn isDeltaTime(&self) {
        todo!()
    }
    pub(crate) fn matchedNoteOff(&self, other: ) {
        todo!()
    }
}