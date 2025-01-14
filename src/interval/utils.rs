use super::{direction::Direction, intervalexception::IntervalException};
use crate::{
    common::{numbertools::Ordinal, types::IntegerOrString},
    defaults::IntegerType,
};

pub(crate) fn convert_generic(value: IntegerOrString) -> Result<IntegerType, IntervalException> {
    let mut direction_scalar = Direction::ASCENDING;
    let post = match value {
        IntegerOrString::Int(value) => value,
        IntegerOrString::String(value) => {
            let value = value.as_str();
            let post: IntegerType;
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
                Some(ordinal) => post = ordinal as IntegerType,
                None => {
                    return Err(IntervalException::new(
                        format!("Cannot convert {} to an interval.", value).to_owned(),
                    ));
                }
            };
            post
        }
    };
    Ok(post * direction_scalar as IntegerType)
}
