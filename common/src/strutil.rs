use serde_json::Value;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum StringType {
    Hex,
    Decimal,
    String
}

pub struct TypeError(String);

impl Error for TypeError {}

impl Debug for TypeError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Display for TypeError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub fn check_type(value: Value) -> Result<StringType, TypeError> {
    match value.as_str() {
        None => {
            Err(TypeError { 0: "Type of given value is not `String`".to_string() })
        }
        Some(v) => {
            if v.starts_with("0x") { return Ok(StringType::Hex); }
            if v.parse::<u64>().is_ok() { return Ok(StringType::Decimal); }
            return Ok(StringType::String);
        }
    }
}