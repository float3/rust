use crate::{
    common::types::OffsetQL,
    defaults::{FloatType, IntegerType, StringType},
};

use super::{consts::ORDINAL_TYPE_FROM_NUM, durationexception::DurationException};

pub(crate) struct DurationTuple {
    pub(crate) r#type: StringType,
    pub(crate) dots: IntegerType,
    pub(crate) quarter_length: OffsetQL,
}

impl DurationTuple {
    pub(crate) fn augment_or_diminish(&self, amount_to_scale: FloatType) -> DurationTuple {
        durationtuple_from_length(self.quarter_length * amount_to_scale)
    }

    pub(crate) fn ordinal(&self) -> Result<DurationException, usize> {
        ORDINAL_TYPE_FROM_NUM
            .iter()
            .position(|&x| x == self.r#type)
            .ok_or(DurationException::TypeNotFound)
    }
}

pub(crate) fn durationtuple_from_length(ql: OffsetQL) -> DurationTuple {
    todo!()
}
