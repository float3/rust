use crate::{
    common::types::Octave,
    defaults::IntegerType,
    interval::{interval::Interval, stepname::StepName},
    pitch::pitch::Pitch,
};

use super::{noteexception::NoteException, notrest::NotRest};

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Note {
    notrest: NotRest,
    is_note: bool,
    pitch: Pitch,
}

impl Note {
    pub(crate) fn new(pitch: Option<StringOrIntegerOrPitch>) -> Self {
        todo!()
    }

    pub(crate) fn _repr_internal(&self) {
        todo!()
    }

    pub(crate) fn __lt__<T>(&self, other: T) -> bool
    where
        T: PitchTrait,
    {
        self.pitch() < other.pitch()
    }

    pub(crate) fn __gt__(&self, other: T) -> bool
    where
        T: PitchTrait,
    {
        self.pitch() > other.pitch()
    }

    pub(crate) fn __le__(&self, other: T) -> bool
    where
        T: PitchTrait,
    {
        self.pitch() <= other.pitch()
    }

    pub(crate) fn __ge__(&self, other: T) -> bool
    where
        T: PitchTrait,
    {
        self.pitch() >= other.pitch()
    }

    pub(crate) fn name(&self) -> String {
        todo!()
    }

    pub(crate) fn set_name(&self, value: String) {
        let _ = value;
        todo!()
    }

    pub(crate) fn name_with_octave(&self) -> String {
        todo!()
    }

    pub(crate) fn set_name_with_octave(&self, value: String) {
        let _ = value;
        todo!()
    }

    pub(crate) fn step(&self) {
        todo!()
    }

    pub(crate) fn set_step(&mut self, value: StepName) {
        self.pitch.set_step(value);
    }

    pub(crate) fn octave(&self) -> Octave {
        self.pitch.octave()
    }

    pub(crate) fn set_octave(&self, value: Octave) {
        self.pitch.set_octave(value);
    }
    pub(crate) fn pitches(&self) -> (Pitch,) {
        (self.pitch.clone(),)
    }

    pub(crate) fn set_pitches(&mut self, value: Vec<Pitch>) -> Result<(), NoteException> {
        match !value.is_empty() {
            true => {
                self.pitch = value[0].clone();
                Ok(())
            }
            false => {
                return Err(NoteException::new(
                    "Cannot set a note to have no pitches".to_owned(),
                ));
            }
        }
    }
    pub(crate) fn transpose(&self, value: T) -> Note
    where
        T: IntervalBase,
    {
        todo!()
    }

    pub(crate) fn transpose_in_place(&mut self, value: T)
    where
        T: IntervalBase,
    {
        todo!()
    }
    pub(crate) fn full_name(&self) -> String {
        todo!()
    }
    pub(crate) fn pitch_changed(&self) {
        todo!()
    }
}
