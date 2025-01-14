use super::{duration::Duration, durationtuple::DurationTuple};
use crate::{
    common::types::{IntegerOrFloat, StringOrIntegerOrFloatOrFraction},
    defaults::{IntegerType, StringType},
    prebase::protom21object::ProtoM21Object,
};

pub(crate) struct Tuplet {
    protom21object: ProtoM21Object,
}

enum DurationTupleOrDurationOrStringOrStringAndInt {
    DurationTuple(DurationTuple),
    Duration(Duration),
    String(StringType),
    StringAndInt((StringType, IntegerType)),
}
enum DurationTupleOrDurationOrStr {
    DurationTuple(DurationTuple),
    Duration(Duration),
    String(StringType),
}

impl Tuplet {
    pub(crate) fn new(
        number_notes_actual: Option<IntegerType>,
        number_notes_normal: Option<IntegerType>,
        duration_actual: DurationTupleOrDurationOrStringOrStringAndInt,
    ) {
        todo!()
    }
    pub(crate) fn __eq__(&self, other: Tuplet) -> bool {
        todo!()
    }
    pub(crate) fn _repr_internal(&self) {
        todo!()
    }
    pub(crate) fn _check_frozen(&self) {
        todo!()
    }
    pub(crate) fn augment_or_diminish(&self, amount_to_scale: IntegerOrFloat) {
        todo!()
    }
    pub(crate) fn set_duration_type(
        &self,
        dur_type: StringOrIntegerOrFloatOrFraction,
        dots: Option<IntegerType>,
    ) {
        todo!()
    }
    pub(crate) fn set_ratio(&self, actual: IntegerType, normal: IntegerType) {
        todo!()
    }
    pub(crate) fn total_tuplet_length(&self) {
        todo!()
    }
    pub(crate) fn tuplet_multiplier(&self) {
        todo!()
    }
    pub(crate) fn duration_actual(&self) -> Option<DurationTuple> {
        todo!()
    }
    pub(crate) fn set_duration_actual(&self, d_a: Option<DurationTupleOrDurationOrStr>) {
        todo!()
    }
    pub(crate) fn duration_normal(&self) -> Option<DurationTuple> {
        todo!()
    }
    pub(crate) fn set_duration_normal(&self, d_n: Option<DurationTupleOrDurationOrStr>) {
        todo!()
    }
    pub(crate) fn full_name(&self) -> StringType {
        todo!()
    }
    pub(crate) fn tuplet_actual(&self) -> (Option<IntegerType>, Option<DurationTuple>) {
        (self.numberNotesActual, self.duration_actual())
    }

    pub(crate) fn set_tuplet_actual(
        &mut self,
        tup_list: (Option<IntegerType>, Option<DurationTupleOrDurationOrStr>),
    ) {
        self._check_frozen();
        self.numberNotesActual = tup_list.0;
        self.set_duration_actual(tup_list.1);
    }

    pub(crate) fn tuplet_normal(&self) -> (Option<IntegerType>, Option<DurationTuple>) {
        (self.numberNotesNormal, self.duration_normal())
    }

    pub(crate) fn set_tuplet_normal(
        &mut self,
        tup_list: (Option<IntegerType>, Option<DurationTupleOrDurationOrStr>),
    ) {
        self._check_frozen();
        self.numberNotesNormal = tup_list.0;
        self.set_duration_normal(tup_list.1);
    }
}
