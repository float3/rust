use crate::defaults::IntegerType;

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
    pub(crate) fn step_to_dnn_offset_reverse(n: IntegerType) -> Self {
        match n {
            0 => Self::C,
            1 => Self::D,
            2 => Self::E,
            3 => Self::F,
            4 => Self::G,
            5 => Self::A,
            6 => Self::B,
            _ => panic!(),
        }
    }

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

    pub(crate) fn step_ref(&self) -> StepName {
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

    pub(crate) fn step_ref_reverse(n: StepName) -> Self {
        match n {
            0 => StepName::C,
            2 => StepName::D,
            4 => StepName::E,
            5 => StepName::F,
            7 => StepName::G,
            9 => StepName::A,
            11 => StepName::B,
            _ => panic!(),
        }
    }
}
