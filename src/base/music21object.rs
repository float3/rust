use crate::{
    defaults::{FloatType, IntegerType},
    duration::duration::Duration,
    prebase::protom21object::ProtoM21Object,
};

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Music21Object {
    protom21object: ProtoM21Object,
    class_sort_order: FloatType,
    is_stream: bool,
    // _styleClass: Style,
    // equalityAttributes: ,

    // activeSite: ,
    // priority: ,
    // seconds: ,
}

impl Music21Object {
    pub(crate) fn new(id: Option<IntegerType>) -> Music21Object {
        Music21Object {
            protom21object: ProtoM21Object::new(id),
            class_sort_order: 20.0,
            is_stream: false,
            // _styleClass: Style,
            // equalityAttributes: ,

            // activeSite: ,
            // priority: ,
            // seconds: ,
        }
    }
    pub(crate) fn id(&self) -> Option<IntegerType> {
        self.protom21object.id()
    }
    pub(crate) fn set_id(&mut self, new_id: IntegerType) {
        self.protom21object.set_id(new_id);
    }

    pub(crate) fn merge_attributes(&self, other: Music21Object) {
        let _ = other;
        todo!()
    }
    // pub(crate) fn _deepcopySubclassable(&self, memo: ) {
    //     todo!()
    // }
    // pub(crate) fn __deepcopy__(&self, memo: ) {
    //     todo!()
    // }
    // pub(crate) fn __getstate__(&self) {
    //     todo!()
    // }
    // pub(crate) fn __setstate__(&self, state: ) {
    //     todo!()
    // }
    // pub(crate) fn _reprInternal(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn hasEditorialInformation(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn editorial(&self) -> Editorial {
    //     todo!()
    // }
    // pub(crate) fn editorial(&self, ed: Editorial) {
    //     todo!()
    // }
    // pub(crate) fn hasStyleInformation(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn style(&self) -> Style {
    //     todo!()
    // }
    // pub(crate) fn style(&self, newStyle: Style) {
    //     todo!()
    // }
    // pub(crate) fn quarterLength(&self) {
    //     todo!()
    // }
    // pub(crate) fn quarterLength(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn derivation(&self) -> Derivation {
    //     todo!()
    // }
    // pub(crate) fn derivation(&self, newDerivation: ) {
    //     todo!()
    // }
    pub(crate) fn clear_cache(&self) {
        todo!()
    }
    // pub(crate) fn getOffsetBySite(&self, site: ) {
    //     todo!()
    // }
    // pub(crate) fn getOffsetBySite(&self, site: ) {
    //     todo!()
    // }
    // pub(crate) fn getOffsetBySite(&self, site: ) {
    //     todo!()
    // }
    // pub(crate) fn setOffsetBySite(&self, site: ) {
    //     todo!()
    // }
    // pub(crate) fn getOffsetInHierarchy(&self, site: ) {
    //     todo!()
    // }
    // pub(crate) fn getSpannerSites(&self, spannerClassList: ) {
    //     todo!()
    // }
    // pub(crate) fn purgeOrphans(&self, excludeStorageStreams: ) {
    //     todo!()
    // }
    // pub(crate) fn purgeLocations(&self, rescanIsDead: ) {
    //     todo!()
    // }
    // pub(crate) fn getContextByClass(&self, className: ) {
    //     todo!()
    // }
    // pub(crate) fn getContextByClass(&self, className: ) {
    //     todo!()
    // }
    // pub(crate) fn getContextByClass(&self, className: ) {
    //     todo!()
    // }
    // pub(crate) fn contextSites(&self) {
    //     todo!()
    // }
    // pub(crate) fn contextSites(&self) {
    //     todo!()
    // }
    // pub(crate) fn contextSites(&self) {
    //     todo!()
    // }
    // pub(crate) fn getAllContextsByClass(&self, className: ) {
    //     todo!()
    // }
    // pub(crate) fn next(&self, className: ) {
    //     todo!()
    // }
    // pub(crate) fn previous(&self, className: ) {
    //     todo!()
    // }
    // pub(crate) fn _getActiveSite(&self) {
    //     todo!()
    // }
    // pub(crate) fn _setActiveSite(&self, site: ) {
    //     todo!()
    // }
    // pub(crate) fn offset(&self) {
    //     todo!()
    // }
    // pub(crate) fn offset(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn sortTuple(&self, useSite: , raiseExceptionOnMiss: bool) -> SortTuple {
    //     todo!()
    // }
    pub(crate) fn duration(&self) -> Duration {
        todo!()
    }
    pub(crate) fn set_duration(&self, durationObj: Duration) {
        todo!()
    }
    // pub(crate) fn informSites(&self, changedInformation: ) {
    //     todo!()
    // }
    // pub(crate) fn _getPriority(&self) {
    //     todo!()
    // }
    // pub(crate) fn _setPriority(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn write(&self, fmt: ) {
    //     todo!()
    // }
    // pub(crate) fn _reprText(&self) {
    //     todo!()
    // }
    // pub(crate) fn _reprTextLine(&self) {
    //     todo!()
    // }
    // pub(crate) fn show(&self, fmt: ) {
    //     todo!()
    // }
    // pub(crate) fn containerHierarchy(&self) {
    //     todo!()
    // }
    // pub(crate) fn splitAtQuarterLength(&self, quarterLength: ) -> _SplitTuple {
    //     todo!()
    // }
    // pub(crate) fn splitByQuarterLengths(&self, quarterLengthList: ) -> _SplitTuple {
    //     todo!()
    // }
    // pub(crate) fn splitAtDurations(&self) -> _SplitTuple {
    //     todo!()
    // }
    // pub(crate) fn measureNumber(&self) {
    //     todo!()
    // }
    // pub(crate) fn _getMeasureOffset(&self, includeMeasurePadding: ) {
    //     todo!()
    // }
    // pub(crate) fn _getTimeSignatureForBeat(&self) {
    //     todo!()
    // }
    // pub(crate) fn beat(&self) {
    //     todo!()
    // }
    // pub(crate) fn beatStr(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn beatDuration(&self) -> Duration {
    //     todo!()
    // }
    // pub(crate) fn beatStrength(&self) -> FloatType {
    //     todo!()
    // }
    // pub(crate) fn _getSeconds(&self) -> FloatType {
    //     todo!()
    // }
    // pub(crate) fn _setSeconds(&self, value: ) {
    //     todo!()
    // }
}

pub(crate) trait Music21ObjectTrait {
    fn music21_object(&self) -> Music21Object;
}
