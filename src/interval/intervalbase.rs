// use crate::note::note::Note;

// pub(crate) struct IntervalBase {
//     music21object: Music21Object,
// }

// impl IntervalBase {
//     pub(crate) fn new() -> IntervalBase {
//         IntervalBase {
//             music21object: Music21Object::new(),
//         }
//     }

//     pub(crate) fn transposeNote(&self, note1: Note) -> Note {
//         todo!()
//     }

//     pub(crate) fn transposePitch(&self, pitch1: Pitch, inplace: bool) -> Pitch {
//         todo!()
//     }

//     pub(crate) fn reverse(&self) {
//         todo!()
//     }

//     pub(crate) fn clear_cache(&self) {
//         todo!()
//     }
// }

use crate::{note::note::Note, pitch::pitch::Pitch};

pub(crate) trait IntervalBase {
    fn transpose_note(&self, note1: Note) -> Note;
    fn transpose_pitch(&self, pitch1: Pitch, inplace: bool) -> Pitch;
    fn reverse(&self);
    fn clear_cache(&self);
}
