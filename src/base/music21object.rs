use crate::prebase::protom21object::ProtoM21Object;

#[derive(PartialEq)]
pub struct Music21Object {
    protom21object: ProtoM21Object,
    class_sort_order: f32,
    is_stream: bool,
    // _styleClass: Style,
    // equalityAttributes: ,

    // activeSite: ,
    // priority: ,
    // seconds: ,
}

impl Music21Object {
    pub fn new(&self, id: i32) -> Music21Object {
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
    pub fn id(&self) {
        todo!()
    }
    pub fn set_id(&mut self, new_id: i32) {
        self.protom21object.set_id(new_id);
    }

    pub fn mergeAttributes(&self, other: Music21Object) {
        todo!()
    }
    // pub fn _deepcopySubclassable(&self, memo: ) {
    //     todo!()
    // }
    // pub fn __deepcopy__(&self, memo: ) {
    //     todo!()
    // }
    // pub fn __getstate__(&self) {
    //     todo!()
    // }
    // pub fn __setstate__(&self, state: ) {
    //     todo!()
    // }
    // pub fn _reprInternal(&self) -> String {
    //     todo!()
    // }
    // pub fn hasEditorialInformation(&self) -> bool {
    //     todo!()
    // }
    // pub fn editorial(&self) -> Editorial {
    //     todo!()
    // }
    // pub fn editorial(&self, ed: Editorial) {
    //     todo!()
    // }
    // pub fn hasStyleInformation(&self) -> bool {
    //     todo!()
    // }
    // pub fn style(&self) -> Style {
    //     todo!()
    // }
    // pub fn style(&self, newStyle: Style) {
    //     todo!()
    // }
    // pub fn quarterLength(&self) {
    //     todo!()
    // }
    // pub fn quarterLength(&self, value: ) {
    //     todo!()
    // }
    // pub fn derivation(&self) -> Derivation {
    //     todo!()
    // }
    // pub fn derivation(&self, newDerivation: ) {
    //     todo!()
    // }
    // pub fn clearCache(&self) {
    //     todo!()
    // }
    // pub fn getOffsetBySite(&self, site: ) {
    //     todo!()
    // }
    // pub fn getOffsetBySite(&self, site: ) {
    //     todo!()
    // }
    // pub fn getOffsetBySite(&self, site: ) {
    //     todo!()
    // }
    // pub fn setOffsetBySite(&self, site: ) {
    //     todo!()
    // }
    // pub fn getOffsetInHierarchy(&self, site: ) {
    //     todo!()
    // }
    // pub fn getSpannerSites(&self, spannerClassList: ) {
    //     todo!()
    // }
    // pub fn purgeOrphans(&self, excludeStorageStreams: ) {
    //     todo!()
    // }
    // pub fn purgeLocations(&self, rescanIsDead: ) {
    //     todo!()
    // }
    // pub fn getContextByClass(&self, className: ) {
    //     todo!()
    // }
    // pub fn getContextByClass(&self, className: ) {
    //     todo!()
    // }
    // pub fn getContextByClass(&self, className: ) {
    //     todo!()
    // }
    // pub fn contextSites(&self) {
    //     todo!()
    // }
    // pub fn contextSites(&self) {
    //     todo!()
    // }
    // pub fn contextSites(&self) {
    //     todo!()
    // }
    // pub fn getAllContextsByClass(&self, className: ) {
    //     todo!()
    // }
    // pub fn next(&self, className: ) {
    //     todo!()
    // }
    // pub fn previous(&self, className: ) {
    //     todo!()
    // }
    // pub fn _getActiveSite(&self) {
    //     todo!()
    // }
    // pub fn _setActiveSite(&self, site: ) {
    //     todo!()
    // }
    // pub fn offset(&self) {
    //     todo!()
    // }
    // pub fn offset(&self, value: ) {
    //     todo!()
    // }
    // pub fn sortTuple(&self, useSite: , raiseExceptionOnMiss: bool) -> SortTuple {
    //     todo!()
    // }
    // pub fn duration(&self) -> Duration {
    //     todo!()
    // }
    // pub fn duration(&self, durationObj: Duration) {
    //     todo!()
    // }
    // pub fn informSites(&self, changedInformation: ) {
    //     todo!()
    // }
    // pub fn _getPriority(&self) {
    //     todo!()
    // }
    // pub fn _setPriority(&self, value: ) {
    //     todo!()
    // }
    // pub fn write(&self, fmt: ) {
    //     todo!()
    // }
    // pub fn _reprText(&self) {
    //     todo!()
    // }
    // pub fn _reprTextLine(&self) {
    //     todo!()
    // }
    // pub fn show(&self, fmt: ) {
    //     todo!()
    // }
    // pub fn containerHierarchy(&self) {
    //     todo!()
    // }
    // pub fn splitAtQuarterLength(&self, quarterLength: ) -> _SplitTuple {
    //     todo!()
    // }
    // pub fn splitByQuarterLengths(&self, quarterLengthList: ) -> _SplitTuple {
    //     todo!()
    // }
    // pub fn splitAtDurations(&self) -> _SplitTuple {
    //     todo!()
    // }
    // pub fn measureNumber(&self) {
    //     todo!()
    // }
    // pub fn _getMeasureOffset(&self, includeMeasurePadding: ) {
    //     todo!()
    // }
    // pub fn _getTimeSignatureForBeat(&self) {
    //     todo!()
    // }
    // pub fn beat(&self) {
    //     todo!()
    // }
    // pub fn beatStr(&self) -> String {
    //     todo!()
    // }
    // pub fn beatDuration(&self) -> Duration {
    //     todo!()
    // }
    // pub fn beatStrength(&self) -> f64 {
    //     todo!()
    // }
    // pub fn _getSeconds(&self) -> f64 {
    //     todo!()
    // }
    // pub fn _setSeconds(&self, value: ) {
    //     todo!()
    // }
}
