use super::duration::Duration;
use crate::common::objects::frozenobject::FrozenObject;

pub(crate) struct FrozenDuration {
    frozenobject: FrozenObject,
    duration: Duration,
}

impl FrozenDuration {
    pub(crate) fn new() -> FrozenDuration {
        let x = FrozenDuration {
            frozenobject: FrozenObject::new(),
            duration: Duration::new(),
        };

        x.update_components();
        x.update_quarter_length();
        x
    }

    fn update_components(&self) {
        self.duration.update_components();
    }

    fn update_quarter_length(&self) {
        self.duration.update_quarter_length();
    }
}
