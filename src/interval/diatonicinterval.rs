use crate::{
    common::types::IntegerOrString, defaults::FloatType, note::note::Note, pitch::pitch::Pitch,
};

use super::{
    chromaticinterval::ChromaticInterval, direction::Direction, genericinterval::GenericInterval,
    interval::Interval, intervalbase::IntervalBase, intervalexception::IntervalException,
    specifier::Specifier,
};

pub(crate) struct DiatonicInterval {
    specifier: Specifier,
    generic: GenericInterval,
}

impl PartialEq for DiatonicInterval {
    fn eq(&self, other: &Self) -> bool {
        self.generic == other.generic
            && self.specifier == other.specifier
            && self.direction() == other.direction()
    }
}

impl DiatonicInterval {
    pub(crate) fn new(
        specifier: IntegerOrString,
        generic: GenericInterval,
    ) -> Result<DiatonicInterval, IntervalException> {
        let specifier = Specifier::parse_specifier(specifier)?;

        if [Specifier::MAJOR, Specifier::MINOR].contains(&specifier) && generic.perfectable() {
            return Err(IntervalException::new(format!(
                "Cannot create a {} {}",
                specifier.nice_name()?,
                generic.nice_name()
            )));
        }

        if specifier == Specifier::PERFECT && generic.value() == -1 {
            return Err(IntervalException::new(
                "There is no such thing as a descending Perfect Unison".to_string(),
            ));
        }

        Ok(DiatonicInterval { specifier, generic })
    }

    pub(crate) fn _repr_internal(&self) -> String {
        todo!()
    }

    pub(crate) fn __hash__(&self) {
        todo!()
    }
    pub(crate) fn name(&self) -> String {
        todo!()
    }
    pub(crate) fn nice_name(&self) -> String {
        todo!()
    }
    pub(crate) fn specific_name(&self) -> String {
        todo!()
    }
    pub(crate) fn simple_name(&self) -> String {
        todo!()
    }
    pub(crate) fn simple_nice_name(&self) -> String {
        todo!()
    }
    pub(crate) fn semi_simple_name(&self) -> String {
        todo!()
    }
    pub(crate) fn semi_simple_nice_name(&self) -> String {
        todo!()
    }
    pub(crate) fn direction(&self) -> Direction {
        todo!()
    }
    pub(crate) fn directed_name(&self) -> String {
        todo!()
    }
    pub(crate) fn directed_nice_name(&self) -> String {
        todo!()
    }
    pub(crate) fn directed_simple_name(&self) -> String {
        todo!()
    }
    pub(crate) fn directed_simple_nice_name(&self) -> String {
        todo!()
    }
    pub(crate) fn directed_semi_simple_name(&self) -> String {
        todo!()
    }
    pub(crate) fn directed_semi_simple_nice_name(&self) -> String {
        todo!()
    }
    pub(crate) fn is_step(&self) -> bool {
        todo!()
    }
    pub(crate) fn is_diatonic_step(&self) -> bool {
        todo!()
    }
    pub(crate) fn is_skip(&self) -> bool {
        todo!()
    }
    pub(crate) fn perfectable(&self) -> bool {
        todo!()
    }
    pub(crate) fn mod7inversion(&self) -> String {
        todo!()
    }
    pub(crate) fn mod7(&self) -> String {
        todo!()
    }
    pub(crate) fn reverse(&self) -> DiatonicInterval {
        todo!()
    }
    pub(crate) fn get_chromatic(&self) -> ChromaticInterval {
        todo!()
    }

    pub(crate) fn transpose_pitch(
        &self,
        pitch1: Pitch,
        interval1: Interval,
        inplace: bool,
    ) -> Pitch {
        let _ = inplace;
        let _ = interval1;
        let _ = pitch1;
        todo!()
    }

    pub(crate) fn specifier_abbreviation(&self) -> String {
        todo!()
    }
    pub(crate) fn cents(&self) -> FloatType {
        todo!()
    }
}

impl IntervalBase for DiatonicInterval {
    fn transpose_note(&self, note1: Note) -> Note {
        let _ = note1;
        todo!()
    }

    fn transpose_pitch(&self, pitch1: Pitch, inplace: bool) -> Pitch {
        let _ = inplace;
        let _ = pitch1;
        todo!()
    }

    fn reverse(&self) {
        todo!()
    }

    fn clear_cache(&self) {
        todo!()
    }
}
