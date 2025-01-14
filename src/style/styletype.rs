#[derive(PartialEq, Clone, Debug)]
pub(crate) enum StyleType {
    BeamStyle,
    BezierStyle,
    Enclosure,
    LineStyle,
    NoteStyle,
    StreamStyle,
    Style,
    StyleMixin,
    TextFormatException,
    TextStyle,
    TextStylePlacement,
}
