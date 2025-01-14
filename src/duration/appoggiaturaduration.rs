use super::{graceduration::GraceDuration, typeorduration::TypeOrDuration};

pub(crate) struct AppoggiaturaDuration {
    graceduration: GraceDuration,
    slash: bool,
    make_time: bool,
}

impl AppoggiaturaDuration {
    pub(crate) fn new(
        type_or_duration: Option<TypeOrDuration>,
        slash: bool,
        make_time: bool,
    ) -> AppoggiaturaDuration {
        AppoggiaturaDuration {
            graceduration: GraceDuration::new(type_or_duration),
            slash,
            make_time,
        }
    }
}
