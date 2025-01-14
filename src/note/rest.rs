use std::collections::HashMap;

use crate::{
    common::types::OffsetQLIn,
    defaults::{IntegerType, StringType},
};

use super::generalnote::GeneralNote;

pub(crate) struct Rest {
    generalnote: GeneralNote,
    is_rest: bool,
    name: String,
    step_shift: IntegerType,
    full_measure: FullMeasure,
}

enum FullMeasure {
    Auto,
    Always,
}

const NAME: &str = "rest";

enum StringOrOffsetQLIn {
    String(StringType),
    OffsetQLIn(OffsetQLIn),
}

impl Rest {
    pub(crate) fn new(
        length: Option<StringOrOffsetQLIn>,
        step_shift: Option<IntegerType>,
        full_measure: Option<FullMeasure>,
    ) -> Rest {
        let mut keywords = HashMap::new();
        if let Some(length) = length {
            match length {
                StringOrOffsetQLIn::String(s) => {
                    keywords.insert("type".to_string(), s.to_string());
                }
                StringOrOffsetQLIn::OffsetQLIn(o) => {
                    keywords.insert("quarterLength".to_string(), o.to_string());
                }
            }
        }

        let step_shift = match step_shift {
            Some(i) => i,
            None => 0,
        };

        let full_measure = match full_measure {
            Some(f) => f,
            None => FullMeasure::Auto,
        };

        Rest {
            generalnote: GeneralNote::new(keywords),
            is_rest: true,
            name: NAME.to_string(),
            step_shift,
            full_measure,
        }
    }
    pub(crate) fn full_name(&self) -> String {
        self.generalnote.full_name()
    }
}
