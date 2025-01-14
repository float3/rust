use crate::{common::types::OffsetQLIn, defaults::StringType};

use super::durationtuple::DurationTuple;

pub(crate) enum TypeOrDuration {
    String(StringType),
    OffsetQLIn(OffsetQLIn),
    DurationTuple(DurationTuple),
}
