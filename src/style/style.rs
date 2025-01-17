use crate::prebase::protom21object::ProtoM21Object;

use super::enclosure::Enclosure;

#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Style {
    protom21object: ProtoM21Object,
    relative_x: Option<FloatType>,
    relative_y: Option<FloatType>,
    absolute_x: Option<FloatType>,
    _absolute_y: Option<FloatType>,
    _enclosure: Option<Enclosure>,
}

impl Style {
    pub(crate) fn new() -> Style {
        Style {
            protom21object: ProtoM21Object::new_without_id(),
            relative_x: None,
            relative_y: None,
            absolute_x: None,
            _absolute_y: None,
            _enclosure: None,
        }
    }

    pub(crate) fn enclosure(&self) -> &Option<Enclosure> {
        &self._enclosure
    }
    pub(crate) fn set_enclosure(&mut self, value: Option<Enclosure>) {
        self._enclosure = value;
    }

    pub(crate) fn _get_absolute_y(&self) -> Option<FloatType> {
        self._absolute_y.clone()
    }
    pub(crate) fn _set_absolute_y(&mut self, value: Option<FloatType>) {
        self._absolute_y = value;
    }
    pub(crate) fn _set_absolute_y_from_string(&mut self, value: &str) {
        match value {
            "above" => self._absolute_y = Some(10.0),
            "below" => self._absolute_y = Some(-70.0),
            _ => self._set_absolute_y(value.parse::<FloatType>().ok()),
        }
    }
}
