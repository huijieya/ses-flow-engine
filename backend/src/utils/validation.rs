use serde_json::Value;
use crate::core::error::{Result, SesError};

pub fn validate_required(value: &Value, field: &str) -> Result<()> {
    if value.get(field).is_none() {
        return Err(SesError::Validation(format!("Field '{}' is required", field)));
    }
    Ok(())
}

pub fn validate_string(value: &Value, field: &str) -> Result<String> {
    value
        .get(field)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| SesError::Validation(format!("Field '{}' must be a string", field)))
}

pub fn validate_integer(value: &Value, field: &str) -> Result<i64> {
    value
        .get(field)
        .and_then(|v| v.as_i64())
        .ok_or_else(|| SesError::Validation(format!("Field '{}' must be an integer", field)))
}

pub fn validate_array(value: &Value, field: &str) -> Result<&Vec<Value>> {
    value
        .get(field)
        .and_then(|v| v.as_array())
        .ok_or_else(|| SesError::Validation(format!("Field '{}' must be an array", field)))
}

pub fn validate_object(value: &Value, field: &str) -> Result<&serde_json::Map<String, Value>> {
    value
        .get(field)
        .and_then(|v| v.as_object())
        .ok_or_else(|| SesError::Validation(format!("Field '{}' must be an object", field)))
}
