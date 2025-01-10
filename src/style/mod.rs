pub(crate)  mod beamstyle;
pub(crate)  mod bezierstyle;
pub(crate)  mod enclosure;
pub(crate)  mod linestyle;
pub(crate)  mod notestyle;
pub(crate)  mod streamstyle;
pub(crate)  mod style;
pub(crate)  mod stylemixin;
pub(crate)  mod textformatexception;
pub(crate)  mod textstyle;
pub(crate)  mod textstyleplacement;

#[derive(PartialEq, Clone, Debug)]
pub(crate)  enum StyleType {
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
