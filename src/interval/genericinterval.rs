use crate::{base::music21object, common::enums::intstring::IntString};

use super::{
    intervalbase::IntervalBase, intervalexception::IntervalException, utils::convert_generic,
};

pub(crate) struct GenericInterval {
    intervalbase: IntervalBase,
    _value: i32,
}

impl PartialEq for GenericInterval {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl GenericInterval {
    pub(crate) fn new(value: IntString) -> Result<GenericInterval, IntervalException> {
        Ok(GenericInterval {
            intervalbase: IntervalBase::new(),
            _value: match convert_generic(value) {
                Ok(val) => val,
                Err(e) => return Err(e),
            },
        })
    }

    // pub(crate)  fn new(&self, value: ) {
    //     todo!()
    // }
    // pub(crate)  fn _reprInternal(&self) {
    //     todo!()
    // }
    // pub(crate)  fn __eq__(&self, other: ) {
    //     todo!()
    // }
    // pub(crate)  fn __hash__(&self) {
    //     todo!()
    // }
    pub(crate) fn value(&self) -> i32 {
        self._value
    }
    pub(crate) fn set_value(&mut self, new_value: i32) -> Result<(), IntervalException> {
        self.clear_cache();
        if new_value == 0 {
            return Err(IntervalException::new(
                "The Zeroth is not an interval".to_string(),
            ));
        }
        self._value = new_value;
        Ok(())
    }
    pub(crate) fn clear_cache(&self) {
        self.intervalbase.clear_cache()
    }
    // pub(crate)  fn directed(&self) -> i32 {
    //     todo!()
    // }
    // pub(crate)  fn directed(&self, newValue: i32) {
    //     todo!()
    // }
    pub(crate) fn undirected(&self) -> i32 {
        todo!()
    }
    // pub(crate)  fn direction(&self) -> Direction {
    //     todo!()
    // }
    // pub(crate)  fn isSkip(&self) -> bool {
    //     todo!()
    // }
    // pub(crate)  fn isDiatonicStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate)  fn isStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate)  fn isUnison(&self) -> bool {
    //     todo!()
    // }
    // pub(crate)  fn _simpleStepsAndOctaves(&self) {
    //     todo!()
    // }
    pub(crate) fn simple_undirected(&self) -> i32 {
        todo!()
    }
    // pub(crate)  fn semiSimpleUndirected(&self) -> i32 {
    //     todo!()
    // }
    // pub(crate)  fn undirectedOctaves(&self) -> i32 {
    //     todo!()
    // }
    // pub(crate)  fn octaves(&self) -> i32 {
    //     todo!()
    // }
    // pub(crate)  fn simpleDirected(&self) -> i32 {
    //     todo!()
    // }
    // pub(crate)  fn semiSimpleDirected(&self) -> i32 {
    //     todo!()
    // }
    pub(crate) fn perfectable(&self) -> bool {
        todo!()
    }
    pub(crate) fn _name_from_int(&self, keyVal: i32) -> String {
        todo!()
    }
    pub(crate) fn nice_name(&self) -> String {
        self._name_from_int(self.undirected())
    }
    pub(crate) fn simple_nice_name(&self) -> String {
        self._name_from_int(self.simple_undirected())
    }
    // pub(crate)  fn semiSimpleNiceName(&self) -> String {
    //     todo!()
    // }
    // pub(crate)  fn staffDistance(&self) -> i32 {
    //     todo!()
    // }
    // pub(crate)  fn mod7inversion(&self) -> i32 {
    //     todo!()
    // }
    // pub(crate)  fn mod7(&self) -> i32 {
    //     todo!()
    // }
    // pub(crate)  fn complement(&self) -> GenericInterval {
    //     todo!()
    // }
    // pub(crate)  fn reverse(&self) -> GenericInterval {
    //     todo!()
    // }
    // pub(crate)  fn transposePitch(&self, p: ) {
    //     todo!()
    // }
    // pub(crate)  fn transposePitchKeyAware(&self, p: ) {
    //     todo!()
    // }
    // pub(crate)  fn getDiatonic(&self, specifier: ) -> DiatonicInterval {
    //     todo!()
    // }
}
