use crate::defaults::IntegerType;
use std::convert::TryFrom;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StepName {
    C = 0,
    D = 1,
    E = 2,
    F = 3,
    G = 4,
    A = 5,
    B = 6,
}

impl StepName {
    pub(crate) fn step_to_dnn_offset(&self) -> IntegerType {
        match self {
            StepName::C => 1,
            StepName::D => 2,
            StepName::E => 3,
            StepName::F => 4,
            StepName::G => 5,
            StepName::A => 6,
            StepName::B => 7,
        }
    }

    pub(crate) fn step_ref(&self) -> IntegerType {
        match self {
            StepName::C => 0,
            StepName::D => 2,
            StepName::E => 4,
            StepName::F => 5,
            StepName::G => 7,
            StepName::A => 9,
            StepName::B => 11,
        }
    }
}

impl TryFrom<IntegerType> for StepName {
    type Error = &'static str;

    fn try_from(value: IntegerType) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StepName::C),
            1 => Ok(StepName::D),
            2 => Ok(StepName::E),
            3 => Ok(StepName::F),
            4 => Ok(StepName::G),
            5 => Ok(StepName::A),
            6 => Ok(StepName::B),
            _ => Err("Invalid value for StepName"),
        }
    }
}

impl From<StepName> for IntegerType {
    fn from(step: StepName) -> Self {
        step as IntegerType
    }
}

impl StepName {
    pub(crate) fn step_ref_reverse(n: IntegerType) -> Result<Self, &'static str> {
        match n {
            0 => Ok(StepName::C),
            2 => Ok(StepName::D),
            4 => Ok(StepName::E),
            5 => Ok(StepName::F),
            7 => Ok(StepName::G),
            9 => Ok(StepName::A),
            11 => Ok(StepName::B),
            _ => Err("Invalid value for StepName"),
        }
    }
}
