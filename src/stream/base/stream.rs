pub struct Stream {
    core::streamcore: core::StreamCore,
    isStream: bool,
    isMeasure: bool,
    classSortOrder: ,
    recursionType: RecursionType,
    _styleClass: ,

    finalBarline: ,
    seconds: ,
    secondsMap: ,
    metadata: ,
}

impl Stream {
    pub fn new() -> Stream {
        Stream {
            core::streamcore: core::StreamCore::new(),
            isStream: todo!(),
            isMeasure: todo!(),
            classSortOrder: todo!(),
            recursionType: todo!(),
            _styleClass: todo!(),

            finalBarline: todo!(),
            seconds: todo!(),
            secondsMap: todo!(),
            metadata: todo!(),
        }
    }
    
    pub fn new(&self, givenElements: ) {
        todo!()
    }
    pub fn __eq__(&self, other: ) {
        todo!()
    }
    pub fn __hash__(&self) -> i32 {
        todo!()
    }
    pub fn _reprInternal(&self) -> String {
        todo!()
    }
    pub fn write(&self, fmt: ) {
        todo!()
    }
    pub fn show(&self, fmt: ) {
        todo!()
    }
    pub fn __len__(&self) -> i32 {
        todo!()
    }
    pub fn __iter__(&self) {
        todo!()
    }
    pub fn iter(&self) {
        todo!()
    }
    pub fn __getitem__(&self, k: String) {
        todo!()
    }
    pub fn __getitem__(&self, k: i32) {
        todo!()
    }
    pub fn __getitem__(&self, k: ) {
        todo!()
    }
    pub fn __getitem__(&self, k: ) {
        todo!()
    }
    pub fn __getitem__(&self, k: ) {
        todo!()
    }
    pub fn __getitem__(&self, k: ) {
        todo!()
    }
    pub fn __getitem__(&self, k: ) {
        todo!()
    }
    pub fn first(&self) {
        todo!()
    }
    pub fn last(&self) {
        todo!()
    }
    pub fn __contains__(&self, el: ) -> bool {
        todo!()
    }
    pub fn elements(&self) {
        todo!()
    }
    pub fn elements(&self, value: ) {
        todo!()
    }
    pub fn __setitem__(&self, k: ) {
        todo!()
    }
    pub fn __delitem__(&self, k: ) {
        todo!()
    }
    pub fn __add__(&self, other: ) {
        todo!()
    }
    pub fn __bool__(&self) {
        todo!()
    }
    pub fn clef(&self) {
        todo!()
    }
    pub fn clef(&self, clefObj: ) {
        todo!()
    }
    pub fn timeSignature(&self) {
        todo!()
    }
    pub fn timeSignature(&self, tsObj: ) {
        todo!()
    }
    pub fn keySignature(&self) {
        todo!()
    }
    pub fn keySignature(&self, keyObj: ) {
        todo!()
    }
    pub fn staffLines(&self) -> i32 {
        todo!()
    }
    pub fn staffLines(&self, newStaffLines: i32) {
        todo!()
    }
    pub fn clear(&self) {
        todo!()
    }
    pub fn cloneEmpty(&self, derivationMethod: ) {
        todo!()
    }
    pub fn mergeAttributes(&self, other: ) {
        todo!()
    }
    pub fn hasElement(&self, obj: ) -> bool {
        todo!()
    }
    pub fn hasElementOfClass(&self, className: ) {
        todo!()
    }
    pub fn mergeElements(&self, other: ) {
        todo!()
    }
    pub fn index(&self, el: ) -> i32 {
        todo!()
    }
    pub fn remove(&self, targetOrList: ) {
        todo!()
    }
    pub fn pop(&self, index: ) {
        todo!()
    }
    pub fn _removeIteration(&self, streamIterator: ) {
        todo!()
    }
    pub fn removeByClass(&self, classFilterList: ) {
        todo!()
    }
    pub fn removeByNotOfClass(&self, classFilterList: ) {
        todo!()
    }
    pub fn _deepcopySubclassable(&self, memo: ) {
        todo!()
    }
    pub fn __deepcopy__(&self, memo: ) {
        todo!()
    }
    pub fn _replaceSpannerBundleForDeepcopy(&self, new: ) {
        todo!()
    }
    pub fn setElementOffset(&self, element: ) {
        todo!()
    }
    pub fn elementOffset(&self, element: ) {
        todo!()
    }
    pub fn insert(&self, offsetOrItemOrList: ) {
        todo!()
    }
    pub fn insertIntoNoteOrChord(&self, offset: ) {
        todo!()
    }
    pub fn append(&self, others: ) {
        todo!()
    }
    pub fn storeAtEnd(&self, itemOrList: ) {
        todo!()
    }
    pub fn insertAndShift(&self, offsetOrItemOrList: ) {
        todo!()
    }
    pub fn setDerivationMethod(&self, derivationMethod: String, recurse: ) {
        todo!()
    }
    pub fn replace(&self, target: ) {
        todo!()
    }
    pub fn splitAtDurations(&self) {
        todo!()
    }
    pub fn splitAtQuarterLength(&self, quarterLength: ) {
        todo!()
    }
    pub fn recurseRepr(&self) -> String {
        todo!()
    }
    pub fn _reprText(&self) -> String {
        todo!()
    }
    pub fn _reprTextLine(&self) -> String {
        todo!()
    }
    pub fn plot(&self, plotFormat: ) {
        todo!()
    }
    pub fn analyze(&self, method: String) {
        todo!()
    }
    pub fn addGroupForElements(&self, group: String, classFilter: ) {
        todo!()
    }
    pub fn getElementsByClass(&self, classFilterList: ) {
        todo!()
    }
    pub fn getElementsByClass(&self, classFilterList: ) {
        todo!()
    }
    pub fn getElementsByClass(&self, classFilterList: ) {
        todo!()
    }
    pub fn getElementsByClass(&self, classFilterList: ) {
        todo!()
    }
    pub fn getElementsNotOfClass(&self, classFilterList: ) {
        todo!()
    }
    pub fn getElementsByGroup(&self, groupFilterList: ) {
        todo!()
    }
    pub fn getElementById(&self, elementId: ) {
        todo!()
    }
    pub fn getElementsByOffset(&self, offsetStart: ) {
        todo!()
    }
    pub fn getElementAtOrBefore(&self, offset: ) {
        todo!()
    }
    pub fn getElementBeforeOffset(&self, offset: ) {
        todo!()
    }
    pub fn getElementAfterElement(&self, element: ) {
        todo!()
    }
    pub fn _getMeasureNumberListByStartEnd(&self, numberStart: ) {
        todo!()
    }
    pub fn measures(&self, numberStart: ) {
        todo!()
    }
    pub fn measure(&self, measureNumber: ) {
        todo!()
    }
    pub fn template(&self) {
        todo!()
    }
    pub fn measureOffsetMap(&self, classFilterList: ) {
        todo!()
    }
    pub fn _getFinalBarline(&self) {
        todo!()
    }
    pub fn _setFinalBarline(&self, value: ) {
        todo!()
    }
    pub fn voices(&self) {
        todo!()
    }
    pub fn spanners(&self) {
        todo!()
    }
    pub fn atSoundingPitch(&self) {
        todo!()
    }
    pub fn atSoundingPitch(&self, value: ) {
        todo!()
    }
    pub fn _transposeByInstrument(&self) {
        todo!()
    }
    pub fn _transposeByInstrument(&self) {
        todo!()
    }
    pub fn _transposeByInstrument(&self) {
        todo!()
    }
    pub fn _treatAtSoundingPitch(&self) {
        todo!()
    }
    pub fn toSoundingPitch(&self) {
        todo!()
    }
    pub fn toWrittenPitch(&self) {
        todo!()
    }
    pub fn getTimeSignatures(&self) {
        todo!()
    }
    pub fn getInstruments(&self) {
        todo!()
    }
    pub fn getInstrument(&self) {
        todo!()
    }
    pub fn invertDiatonic(&self, inversionNote: ) {
        todo!()
    }
    pub fn shiftElements(&self, offset: ) {
        todo!()
    }
    pub fn transferOffsetToElements(&self) {
        todo!()
    }
    pub fn repeatAppend(&self, item: ) {
        todo!()
    }
    pub fn repeatInsert(&self, item: ) {
        todo!()
    }
    pub fn extractContext(&self, searchElement: ) {
        todo!()
    }
    pub fn _uniqueOffsetsAndEndTimes(&self, offsetsOnly: ) {
        todo!()
    }
    pub fn chordify(&self) {
        todo!()
    }
    pub fn splitByClass(&self, classObj: ) {
        todo!()
    }
    pub fn offsetMap(&self, srcObj: ) {
        todo!()
    }
    pub fn makeMeasures(&self, meterStream: ) {
        todo!()
    }
    pub fn makeRests(&self, refStreamOrTimeRange: ) {
        todo!()
    }
    pub fn makeTies(&self, meterStream: ) {
        todo!()
    }
    pub fn makeBeams(&self) {
        todo!()
    }
    pub fn makeAccidentals(&self) {
        todo!()
    }
    pub fn haveAccidentalsBeenMade(&self) {
        todo!()
    }
    pub fn makeNotation(&self) {
        todo!()
    }
    pub fn extendDuration(&self, objClass: ) {
        todo!()
    }
    pub fn stripTies(&self) {
        todo!()
    }
    pub fn stripTies(&self) {
        todo!()
    }
    pub fn stripTies(&self) {
        todo!()
    }
    pub fn extendTies(&self, ignoreRests: ) {
        todo!()
    }
    pub fn sort(&self, force: ) {
        todo!()
    }
    pub fn sorted(&self) {
        todo!()
    }
    pub fn flatten(&self, retainContainers: ) {
        todo!()
    }
    pub fn flat(&self) {
        todo!()
    }
    pub fn recurse(&self) {
        todo!()
    }
    pub fn recurse(&self) {
        todo!()
    }
    pub fn recurse(&self) {
        todo!()
    }
    pub fn containerInHierarchy(&self, el: ) {
        todo!()
    }
    pub fn makeImmutable(&self) {
        todo!()
    }
    pub fn makeMutable(&self, recurse: ) {
        todo!()
    }
    pub fn highestOffset(&self) {
        todo!()
    }
    pub fn _setHighestTime(&self, value: ) {
        todo!()
    }
    pub fn highestTime(&self) {
        todo!()
    }
    pub fn lowestOffset(&self) {
        todo!()
    }
    pub fn _getDuration(&self) {
        todo!()
    }
    pub fn _setDuration(&self, durationObj: ) {
        todo!()
    }
    pub fn duration(&self) {
        todo!()
    }
    pub fn duration(&self, value: ) {
        todo!()
    }
    pub fn _setSeconds(&self, value: ) {
        todo!()
    }
    pub fn _getSeconds(&self) {
        todo!()
    }
    pub fn metronomeMarkBoundaries(&self, srcObj: ) {
        todo!()
    }
    pub fn _accumulatedSeconds(&self, mmBoundaries: ) {
        todo!()
    }
    pub fn _getSecondsMap(&self, srcObj: ) {
        todo!()
    }
    pub fn _getMetadata(&self) {
        todo!()
    }
    pub fn _setMetadata(&self, metadataObj: ) {
        todo!()
    }
    pub fn _getMeasureOffset(&self) {
        todo!()
    }
    pub fn beat(&self) {
        todo!()
    }
    pub fn beatStr(&self) {
        todo!()
    }
    pub fn beatDuration(&self) {
        todo!()
    }
    pub fn beatStrength(&self) {
        todo!()
    }
    pub fn beatAndMeasureFromOffset(&self, searchOffset: ) {
        todo!()
    }
    pub fn transpose(&self) {
        todo!()
    }
    pub fn scaleOffsets(&self, amountToScale: ) {
        todo!()
    }
    pub fn scaleDurations(&self, amountToScale: ) {
        todo!()
    }
    pub fn augmentOrDiminish(&self, amountToScale: ) {
        todo!()
    }
    pub fn quantize(&self, quarterLengthDivisors: , processOffsets: bool, processDurations: bool, inPlace: bool, recurse: bool) {
        todo!()
    }
    pub fn expandRepeats(&self, copySpanners: bool) {
        todo!()
    }
    pub fn sliceByQuarterLengths(&self, quarterLengthList: ) {
        todo!()
    }
    pub fn sliceByGreatestDivisor(&self) {
        todo!()
    }
    pub fn sliceAtOffsets(&self, offsetList: ) {
        todo!()
    }
    pub fn sliceByBeat(&self, target: ) {
        todo!()
    }
    pub fn hasMeasures(&self) {
        todo!()
    }
    pub fn hasVoices(&self) {
        todo!()
    }
    pub fn hasPartLikeStreams(&self) {
        todo!()
    }
    pub fn isTwelveTone(&self) {
        todo!()
    }
    pub fn isWellFormedNotation(&self) -> bool {
        todo!()
    }
    pub fn notesAndRests(&self) {
        todo!()
    }
    pub fn notes(&self) {
        todo!()
    }
    pub fn pitches(&self) {
        todo!()
    }
    pub fn findConsecutiveNotes(&self) {
        todo!()
    }
    pub fn findConsecutiveNotes(&self) {
        todo!()
    }
    pub fn findConsecutiveNotes(&self) {
        todo!()
    }
    pub fn findConsecutiveNotes(&self) {
        todo!()
    }
    pub fn melodicIntervals(&self) {
        todo!()
    }
    pub fn _getDurSpan(&self, flatStream: ) {
        todo!()
    }
    pub fn _durSpanOverlap(&self, a: ) {
        todo!()
    }
    pub fn _findLayering(&self) {
        todo!()
    }
    pub fn _consolidateLayering(&self, layeringMap: ) {
        todo!()
    }
    pub fn findGaps(&self) {
        todo!()
    }
    pub fn isGapless(&self) -> bool {
        todo!()
    }
    pub fn getOverlaps(&self) {
        todo!()
    }
    pub fn isSequence(&self) -> bool {
        todo!()
    }
    pub fn simultaneousAttacks(&self, stream2: ) {
        todo!()
    }
    pub fn attachIntervalsBetweenStreams(&self, cmpStream: ) {
        todo!()
    }
    pub fn attachMelodicIntervals(&self) {
        todo!()
    }
    pub fn playingWhenAttacked(&self, el: ) {
        todo!()
    }
    pub fn allPlayingWhileSounding(&self, el: ) {
        todo!()
    }
    pub fn makeVoices(&self) {
        todo!()
    }
    pub fn _maxVoiceCount(&self) {
        todo!()
    }
    pub fn voicesToParts(&self) {
        todo!()
    }
    pub fn explode(&self) {
        todo!()
    }
    pub fn flattenUnnecessaryVoices(&self) {
        todo!()
    }
    pub fn lyrics(&self, ignoreBarlines: bool, recurse: bool, skipTies: bool) {
        todo!()
    }
    pub fn activateVariants(&self, group: ) {
        todo!()
    }
    pub fn _insertReplacementVariant(&self, v: ) {
        todo!()
    }
    pub fn _insertDeletionVariant(&self, v: ) {
        todo!()
    }
    pub fn _insertInsertionVariant(&self, v: ) {
        todo!()
    }
    pub fn _removeOrExpandGaps(&self, listOffsetDurExemption: ) {
        todo!()
    }
    pub fn _fixMeasureNumbers(&self, deletedMeasures: ) {
        todo!()
    }
    pub fn showVariantAsOssialikePart(&self, containedPart: ) {
        todo!()
    }
}