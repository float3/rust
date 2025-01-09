pub struct MidiEvent {
    prebase::protom21object: prebase::ProtoM21Object,
    pitch: ,
    velocity: ,
    data: ,
}

impl MidiEvent {
    pub fn new() -> MidiEvent {
        MidiEvent {
            prebase::protom21object: prebase::ProtoM21Object::new(),
            pitch: todo!(),
            velocity: todo!(),
            data: todo!(),
        }
    }
    
    pub fn new(&self, track: ) {
        todo!()
    }
    pub fn sortOrder(&self) -> i32 {
        todo!()
    }
    pub fn _reprInternal(&self) {
        todo!()
    }
    pub fn _setPitch(&self, value: ) {
        todo!()
    }
    pub fn _getPitch(&self) {
        todo!()
    }
    pub fn _setVelocity(&self, value: ) {
        todo!()
    }
    pub fn _getVelocity(&self) {
        todo!()
    }
    pub fn _setData(&self, value: ) {
        todo!()
    }
    pub fn _getData(&self) {
        todo!()
    }
    pub fn setPitchBend(&self, cents: ) {
        todo!()
    }
    pub fn parseChannelVoiceMessage(&self, midiBytes: ) {
        todo!()
    }
    pub fn read(&self, midiBytes: ) {
        todo!()
    }
    pub fn getBytes(&self) {
        todo!()
    }
    pub fn isNoteOn(&self) {
        todo!()
    }
    pub fn isNoteOff(&self) {
        todo!()
    }
    pub fn isDeltaTime(&self) {
        todo!()
    }
    pub fn matchedNoteOff(&self, other: ) {
        todo!()
    }
}