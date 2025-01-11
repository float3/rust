use super::{
    intervalbase::IntervalBase, intervalexception::IntervalException, utils::convert_generic,
};
use crate::{
    base::music21object::Music21Object, common::enums::intstring::IntString, defaults::IntegerType,
    note::note::Note, pitch::pitch::Pitch,
};

pub(crate) struct GenericInterval {
    music21object: Music21Object,
    _value: IntegerType,
}

impl PartialEq for GenericInterval {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl GenericInterval {
    pub(crate) fn new(value: IntString) -> Result<GenericInterval, IntervalException> {
        Ok(GenericInterval {
            music21object: Music21Object::new(None),
            _value: match convert_generic(value) {
                Ok(val) => val,
                Err(e) => return Err(e),
            },
        })
    }

    // pub(crate) fn new(&self, value: ) {
    //     todo!()
    // }
    // pub(crate) fn _reprInternal(&self) {
    //     todo!()
    // }
    // pub(crate) fn __eq__(&self, other: ) {
    //     todo!()
    // }
    // pub(crate) fn __hash__(&self) {
    //     todo!()
    // }
    pub(crate) fn value(&self) -> IntegerType {
        self._value
    }
    pub(crate) fn set_value(&mut self, new_value: IntegerType) -> Result<(), IntervalException> {
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
        self.music21object.clear_cache()
    }

    // pub(crate) fn directed(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn directed(&self, newValue: IntegerType) {
    //     todo!()
    // }

    pub(crate) fn undirected(&self) -> IntegerType {
        todo!()
    }

    // pub(crate) fn direction(&self) -> Direction {
    //     todo!()
    // }
    // pub(crate) fn isSkip(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isDiatonicStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isStep(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn isUnison(&self) -> bool {
    //     todo!()
    // }
    // pub(crate) fn _simpleStepsAndOctaves(&self) {
    //     todo!()
    // }
    pub(crate) fn simple_undirected(&self) -> IntegerType {
        todo!()
    }
    // pub(crate) fn semiSimpleUndirected(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn undirectedOctaves(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn octaves(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn simpleDirected(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn semiSimpleDirected(&self) -> IntegerType {
    //     todo!()
    // }
    pub(crate) fn perfectable(&self) -> bool {
        todo!()
    }
    pub(crate) fn _name_from_int(&self, key_val: IntegerType) -> String {
        let _ = key_val;
        todo!()
    }
    pub(crate) fn nice_name(&self) -> String {
        self._name_from_int(self.undirected())
    }
    pub(crate) fn simple_nice_name(&self) -> String {
        self._name_from_int(self.simple_undirected())
    }
    // pub(crate) fn semiSimpleNiceName(&self) -> String {
    //     todo!()
    // }
    // pub(crate) fn staffDistance(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn mod7inversion(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn mod7(&self) -> IntegerType {
    //     todo!()
    // }
    // pub(crate) fn complement(&self) -> GenericInterval {
    //     todo!()
    // }
    // pub(crate) fn reverse(&self) -> GenericInterval {
    //     todo!()
    // }
    // pub(crate) fn transposePitch(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn transposePitchKeyAware(&self, p: ) {
    //     todo!()
    // }
    // pub(crate) fn getDiatonic(&self, specifier: ) -> DiatonicInterval {
    //     todo!()
    // }
}

impl IntervalBase for GenericInterval {
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
