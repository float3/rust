use super::enums::offsetspecial::OffsetSpecial;
use crate::{
    defaults::{FloatType, IntegerType, StringType},
    pitch::{accidental::Accidental, pitch::Pitch},
};
use fraction::Fraction;

pub(crate) type Octave = Option<IntegerType>;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum OffsetQL {
    Float(FloatType),
    Fraction(Fraction),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum OffsetQLSpecial {
    Float(FloatType),
    Fraction(Fraction),
    OffsetSpecial(OffsetSpecial),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum OffsetQLIn {
    Int(IntegerType),
    Float(FloatType),
    Fraction(Fraction),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum IntegerOrString {
    Int(IntegerType),
    String(StringType),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum IntegerOrFloat {
    Int(IntegerType),
    Float(FloatType),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringOrIntegerOrFloat {
    String(StringType),
    Integer(IntegerType),
    Float(FloatType),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringOrIntegerOrFLoatOrAccidental {
    String(StringType),
    Integer(IntegerType),
    Float(FloatType),
    Accidental(Accidental),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringOrIntegerOrPitch {
    String(StringType),
    Integer(IntegerType),
    Pitch(Pitch),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringOrIntegerOrFloatOrFraction {
    String(StringType),
    Integer(IntegerType),
    Float(FloatType),
    Fraction(Fraction),
}
