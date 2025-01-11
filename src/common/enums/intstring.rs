use crate::defaults::IntegerType;

pub(crate) enum IntString {
    Int(IntegerType),
    String(String),
}
