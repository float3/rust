use crate::defaults::FloatType;

use super::{duration::Duration, durationtuple::DurationTuple, typeorduration::TypeOrDuration};

pub(crate) struct GraceDuration {
    duration: Option<Duration>,
    is_grace: bool,
    linked: bool,
    quarter_length: FloatType,
    _make_time: bool,
    _slash: Option<bool>,
}

impl GraceDuration {
    pub(crate) fn new(type_or_duration: Option<TypeOrDuration>) -> GraceDuration {
        let x = GraceDuration {
            duration: Duration::new(type_or_duration),
            is_grace: true,
            linked: false,
            quarter_length: 0.0,
            _make_time: false,
            _slash: None,
        };

        if x.component_needs_updating() {
            x.update_component();
        }

        let mut newcomponents;
        for c in x.components() {
            newcomponents.append(DurationTuple {
                r#type: c.r#type,
                dots: c.dots,
                quarter_length: 0.0,
            });
        }
        x.set_components(newcomponents);

        x.set_slash(Some(true));
        x.set_steal_time_previous(None);
        x.set_steal_time_following(None);
    }
    pub(crate) fn make_time(&self) -> bool {
        todo!()
    }
    pub(crate) fn set_make_time(&self, expr: Option<bool>) {
        todo!()
    }
    pub(crate) fn slash(&self) -> Option<bool> {
        todo!()
    }
    pub(crate) fn set_slash(&self, expr: Option<bool>) {
        todo!()
    }

    fn components(&self) -> Vec<DurationTuple> {
        self.duration.components()
    }

    fn set_components(&self, components: Vec<DurationTuple>) {
        self.duration.set_components(components)
    }

    fn component_needs_updating(&self) -> bool {
        self.duration.component_needs_updating()
    }

    fn update_component(&self) {
        self.duration.update_component()
    }

    fn set_steal_time_previous(&self, expr: Option<bool>) {
        todo!()
    }

    fn set_steal_time_following(&self, expr: Option<bool>) {
        todo!()
    }
}
