#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Direction {
    DESCENDING = -1,
    OBLIQUE = 0,
    ASCENDING = 1,
}

impl Direction {
    pub(crate) fn term(&self) -> String {
        match self {
            Direction::ASCENDING => "Ascending".to_owned(),
            Direction::DESCENDING => "Descending".to_owned(),
            Direction::OBLIQUE => "Oblique".to_owned(),
        }
    }
}
