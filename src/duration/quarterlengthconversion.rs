use super::{durationtuple::DurationTuple, tuplet::Tuplet};

pub(crate) struct QuarterLengthConversion {
    components: Vec<DurationTuple>,
    tuplet: Option<Tuplet>,
}

impl QuarterLengthConversion {
    pub(crate) fn new() -> QuarterLengthConversion {
        QuarterLengthConversion {
            components: Vec::new(),
            tuplet: None,
        }
    }
}
