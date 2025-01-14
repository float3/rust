use std::cmp::Ordering;

use crate::{
    common::types::{Octave, StringOrIntegerOrPitch},
    interval::{intervalbase::IntervalBase, stepname::StepName},
    pitch::pitch::Pitch,
};

use super::{noteexception::NoteException, notrest::NotRest};

#[derive(Clone, Debug)]
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

    pub(crate) fn set_octave(&mut self, value: Octave) {
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
    pub(crate) fn transpose<T>(&self, value: T) -> Note
    where
        T: IntervalBase,
    {
        todo!()
    }

    pub(crate) fn transpose_in_place<T>(&mut self, value: T)
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

impl PartialEq for Note {
    fn eq(&self, other: &Note) -> bool {
        self.pitch == other.pitch
    }
}

impl Eq for Note {}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Note) -> Option<Ordering> {
        self.pitch.partial_cmp(&other.pitch)
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Note) -> Ordering {
        self.pitch.cmp(&other.pitch)
    }
}
