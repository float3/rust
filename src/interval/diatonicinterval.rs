use crate::common::enums::intstring::IntString;

use super::{chromaticinterval::ChromaticInterval, direction::Direction, genericinterval::GenericInterval, intervalbase::IntervalBase, intervalexception::IntervalException, specifier::Specifier};

pub(crate)  struct DiatonicInterval {
    intervalbase: IntervalBase,
    specifier: Specifier,
    generic: GenericInterval,

}

impl PartialEq for DiatonicInterval {
    fn eq(&self, other: &Self) -> bool {
        self.generic == other.generic && self.specifier == other.specifier && self.direction() == other.direction()
    }
}

impl DiatonicInterval {
    pub(crate)  fn new(specifier: IntString, generic: GenericInterval) -> Result<DiatonicInterval, IntervalException> {
        let specifier = Specifier::parse_specifier(specifier)?;

        if [Specifier::MAJOR,Specifier::MINOR].contains(&specifier) && generic.perfectable() {
           return Err(IntervalException::new(format!("Cannot create a {} {}", specifier.nice_name()?, generic.nice_name()?)));
        }

        if specifier == Specifier::PERFECT && generic.value() == -1 {
            return Err(IntervalException::new("There is no such thing as a descending Perfect Unison".to_string()));
        }

        Ok(DiatonicInterval {
            intervalbase: IntervalBase::new(),
            specifier,
            generic
        })
    }
    
    pub(crate)  fn _reprInternal(&self) -> String {
        todo!()
    }

    pub(crate)  fn __hash__(&self) {
        todo!()
    }
    pub(crate)  fn name(&self) -> String {
        todo!()
    }
    pub(crate)  fn niceName(&self) -> String {
        todo!()
    }
    pub(crate)  fn specificName(&self) -> String {
        todo!()
    }
    pub(crate)  fn simpleName(&self) -> String {
        todo!()
    }
    pub(crate)  fn simpleNiceName(&self) -> String {
        todo!()
    }
    pub(crate)  fn semiSimpleName(&self) -> String {
        todo!()
    }
    pub(crate)  fn semiSimpleNiceName(&self) -> String {
        todo!()
    }
    pub(crate)  fn direction(&self) -> Direction {
        todo!()
    }
    pub(crate)  fn directedName(&self) -> String {
        todo!()
    }
    pub(crate)  fn directedNiceName(&self) -> String {
        todo!()
    }
    pub(crate)  fn directedSimpleName(&self) -> String {
        todo!()
    }
    pub(crate)  fn directedSimpleNiceName(&self) -> String {
        todo!()
    }
    pub(crate)  fn directedSemiSimpleName(&self) -> String {
        todo!()
    }
    pub(crate)  fn directedSemiSimpleNiceName(&self) -> String {
        todo!()
    }
    pub(crate)  fn isStep(&self) -> bool {
        todo!()
    }
    pub(crate)  fn isDiatonicStep(&self) -> bool {
        todo!()
    }
    pub(crate)  fn isSkip(&self) -> bool {
        todo!()
    }
    pub(crate)  fn perfectable(&self) -> bool {
        todo!()
    }
    pub(crate)  fn mod7inversion(&self) -> String {
        todo!()
    }
    pub(crate)  fn mod7(&self) -> String {
        todo!()
    }
    pub(crate)  fn reverse(&self) -> DiatonicInterval {
        todo!()
    }
    pub(crate)  fn getChromatic(&self) -> ChromaticInterval {
        todo!()
    }
    pub(crate)  fn transposePitch(&self, p: ) {
        todo!()
    }
    pub(crate)  fn specifierAbbreviation(&self) -> String {
        todo!()
    }
    pub(crate)  fn cents(&self) -> f64 {
        todo!()
    }
}