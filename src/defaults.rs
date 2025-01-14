use crate::interval::stepname::StepName;

pub(crate) type IntegerType = i32;
pub(crate) type FloatType = f64;
pub(crate) type StringType = String;

pub(crate) const PITCH_STEP: StepName = StepName::C;
pub(crate) const PITCH_OCTAVE: IntegerType = 4;

pub(crate) const TWELFTH_ROOT_OF_TWO : FloatType = 2.0f64.powf(1.0 / 12.0);