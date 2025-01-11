use std::collections::HashMap;

use super::intervalexception::IntervalException;
use crate::common::enums::intstring::IntString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Specifier {
    PERFECT = 1,
    MAJOR = 2,
    MINOR = 3,
    AUGMENTED = 4,
    DIMINISHED = 5,
    DBLAUG = 6,
    DBLDIM = 7,
    TRPAUG = 8,
    TRPDIM = 9,
    QUADAUG = 10,
    QUADDIM = 11,
}

struct SpecifierMapping {
    value: IntegerType,
    variants: &'static [&'static str],
    nice_name: &'static str,
}

const SPECIFIER_MAPPINGS: &[SpecifierMapping] = &[
    SpecifierMapping {
        value: 1,
        variants: &["P", "p", "perfect"],
        nice_name: "perfect",
    },
    SpecifierMapping {
        value: 2,
        variants: &["M", "major"],
        nice_name: "major",
    },
    SpecifierMapping {
        value: 3,
        variants: &["m", "minor"],
        nice_name: "minor",
    },
    SpecifierMapping {
        value: 4,
        variants: &["A", "a", "augmented"],
        nice_name: "augmented",
    },
    SpecifierMapping {
        value: 5,
        variants: &["D", "d", "diminished"],
        nice_name: "diminished",
    },
    SpecifierMapping {
        value: 6,
        variants: &["AA", "aa", "doubly-augmented"],
        nice_name: "doubly-augmented",
    },
    SpecifierMapping {
        value: 7,
        variants: &["DD", "dd", "doubly-diminished"],
        nice_name: "doubly-diminished",
    },
    SpecifierMapping {
        value: 8,
        variants: &["AAA", "aaa", "triply-augmented"],
        nice_name: "triply-augmented",
    },
    SpecifierMapping {
        value: 9,
        variants: &["DDD", "ddd", "triply-diminished"],
        nice_name: "triply-diminished",
    },
    SpecifierMapping {
        value: 10,
        variants: &["AAAA", "aaaa", "quadruply-augmented"],
        nice_name: "quadruply-augmented",
    },
    SpecifierMapping {
        value: 11,
        variants: &["DDDD", "dddd", "quadruply-diminished"],
        nice_name: "quadruply-diminished",
    },
];

static STRING_TO_SPECIFIER: Lazy<HashMap<&'static str, Specifier>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for mapping in SPECIFIER_MAPPINGS {
        for &variant in mapping.variants {
            m.insert(
                variant.to_lowercase().as_str(),
                Specifier::try_from(mapping.value).unwrap(),
            );
        }
    }
    m
});

impl TryFrom<IntegerType> for Specifier {
    type Error = IntervalException;

    fn try_from(value: IntegerType) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Specifier::PERFECT),
            2 => Ok(Specifier::MAJOR),
            3 => Ok(Specifier::MINOR),
            4 => Ok(Specifier::AUGMENTED),
            5 => Ok(Specifier::DIMINISHED),
            6 => Ok(Specifier::DBLAUG),
            7 => Ok(Specifier::DBLDIM),
            8 => Ok(Specifier::TRPAUG),
            9 => Ok(Specifier::TRPDIM),
            10 => Ok(Specifier::QUADAUG),
            11 => Ok(Specifier::QUADDIM),
            _ => Err(IntervalException::new(format!(
                "Invalid specifier: {}",
                value
            ))),
        }
    }
}

impl Specifier {
    pub(crate) fn parse_specifier(specifier: IntString) -> Result<Specifier, IntervalException> {
        match specifier {
            IntString::Int(int) => Specifier::try_from(int),
            IntString::String(string) => STRING_TO_SPECIFIER
                .get(&string.to_lowercase()[..])
                .cloned()
                .ok_or_else(|| IntervalException::new(format!("Invalid specifier: {}", string))),
        }
    }

    pub(crate) fn nice_name(&self) -> Result<&'static str, IntervalException> {
        let value = *self as IntegerType;
        SPECIFIER_MAPPINGS
            .iter()
            .find(|mapping| mapping.value == value)
            .map(|mapping| mapping.nice_name)
            .ok_or_else(|| IntervalException::new(format!("Unknown specifier: {}", value)))
    }

    pub(crate) fn __str__(&self) -> String {
        todo!()
    }
    pub(crate) fn __repr__(&self) {
        todo!()
    }

    pub(crate) fn inversion(&self) {
        todo!()
    }
    pub(crate) fn semitones_above_perfect(&self) {
        todo!()
    }
    pub(crate) fn semitones_above_major(&self) {
        todo!()
    }
}
