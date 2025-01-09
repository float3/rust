use crate::prebase::protom21object::ProtoM21Object;

use super::enclosure::Enclosure;

#[derive(PartialEq)]
pub struct Style {
    protom21object: ProtoM21Object,
    relative_x: Option<f32>,
    relative_y: Option<f32>,
    absolute_x: Option<f32>,
    _absolute_y: Option<f32>,
    _enclosure: Option<Enclosure>,
}

impl Style {
    pub fn new() -> Style {
        Style {
            protom21object: ProtoM21Object::new_without_id(),
            relative_x: None,
            relative_y: None,
            absolute_x: None,
            _absolute_y: None,
            _enclosure: None,
        }
    }

    pub fn enclosure(&self) -> &Option<Enclosure> {
        &self._enclosure
    }
    pub fn set_enclosure(&mut self, value: Option<Enclosure>) {
        self._enclosure = value;
    }

    pub fn _get_absolute_y(&self) -> Option<f32> {
        self._absolute_y.clone()
    }
    pub fn _set_absolute_y(&mut self, value: Option<f32>) {
        self._absolute_y = value;
    }
    pub fn _set_absolute_y_from_string(&mut self, value: &str) {
        match value {
            "above" => self._absolute_y = Some(10.0),
            "below" => self._absolute_y = Some(-70.0),
            _ => self._set_absolute_y(value.parse::<f32>().ok()),
        }
    }
}
