use super::{direction::Direction, intervalexception::IntervalException};
use crate::common::{enums::intstring::IntString, numbertools::Ordinal};

pub(crate)  fn convert_generic(value: IntString) -> Result<i32, IntervalException> {
    let mut direction_scalar = Direction::ASCENDING;
    let post = match value {
        IntString::Int(value) => value,
        IntString::String(value) => {
            let value = value.as_str();
            let post: i32;
            let mut value: String = value.trim().to_lowercase();

            // Determine direction

            for direction in [Direction::ASCENDING, Direction::DESCENDING] {
                if direction.term().to_lowercase() == value {
                    direction_scalar = direction;
                    value = value
                        .replace(direction.term().to_lowercase().as_str(), "")
                        .trim()
                        .to_string();
                }
            }

            match Ordinal::from_string(value.as_str()) {
                Some(ordinal) => post = ordinal as i32,
                None => {
                    return Err(IntervalException::new(
                        format!("Cannot convert {} to an interval.", value).to_owned(),
                    ));
                }
            };
            post
        }
    };
    Ok(post * direction_scalar as i32)
}
