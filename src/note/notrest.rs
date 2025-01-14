use super::generalnote::GeneralNote;

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct NotRest {
    generalnote: GeneralNote,
    equality_attributes: &'static [&'static str],
}

impl NotRest {
    pub(crate) fn new() -> NotRest {
        todo!()
    }
    pub(crate) fn stem_direction(&self) -> String {
        todo!()
    }
    // pub(crate) fn stemDirection(&self, direction: ) {
    //     todo!()
    // }
    // pub(crate) fn notehead(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn notehead(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn noteheadFill(&self) {
    //     todo!()
    // }
    // pub(crate) fn noteheadFill(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn noteheadParenthesis(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn noteheadParenthesis(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn hasVolumeInformation(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn _getVolume(&self, forceClient: ) {
    //     todo!()
    // }
    // pub(crate) fn _setVolume(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn volume(&self) {
    //     todo!()
    // }
    // pub(crate) fn volume(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn storedInstrument(&self) {
    //     todo!()
    // }
    // pub(crate) fn storedInstrument(&self, newValue: ) {
    //     todo!()
    // }
    // pub(crate) fn getInstrument(&self) {
    //     todo!()
    // }
    // pub(crate) fn getInstrument(&self) {
    //     todo!()
    // }
    // pub(crate) fn getInstrument(&self) {
    //     todo!()
    // }
}
