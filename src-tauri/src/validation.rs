use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Field '{field}' cannot be empty")]
    EmptyField { field: String },

    #[error("Field '{field}' exceeds maximum length of {max_len} characters")]
    TooLong { field: String, max_len: usize },

    #[error("Field '{field}' is below minimum length of {min_len} characters")]
    TooShort { field: String, min_len: usize },

    #[error("Field '{field}' value {value} is out of range [{min}, {max}]")]
    OutOfRange {
        field: String,
        value: String,
        min: String,
        max: String,
    },

    #[error("Field '{field}' contains invalid characters")]
    InvalidCharacters { field: String },
}

/// Validate that a string is not empty or only whitespace
pub fn validate_not_empty(field: &str, value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::EmptyField {
            field: field.to_string(),
        });
    }
    Ok(())
}

/// Validate string length constraints
pub fn validate_length(
    field: &str,
    value: &str,
    min_len: Option<usize>,
    max_len: Option<usize>,
) -> Result<(), ValidationError> {
    let len = value.len();

    if let Some(min) = min_len {
        if len < min {
            return Err(ValidationError::TooShort {
                field: field.to_string(),
                min_len: min,
            });
        }
    }

    if let Some(max) = max_len {
        if len > max {
            return Err(ValidationError::TooLong {
                field: field.to_string(),
                max_len: max,
            });
        }
    }

    Ok(())
}

/// Validate a number is within a range
pub fn validate_range<T: PartialOrd + ToString>(
    field: &str,
    value: T,
    min: T,
    max: T,
) -> Result<(), ValidationError> {
    if value < min || value > max {
        return Err(ValidationError::OutOfRange {
            field: field.to_string(),
            value: value.to_string(),
            min: min.to_string(),
            max: max.to_string(),
        });
    }
    Ok(())
}

/// Validate temperature parameter (0.0 to 2.0)
pub fn validate_temperature(temp: f32) -> Result<(), ValidationError> {
    validate_range("temperature", temp, 0.0, 2.0)
}

/// Validate top_k parameter (1 to 100)
pub fn validate_top_k(top_k: usize) -> Result<(), ValidationError> {
    validate_range("top_k", top_k, 1, 100)
}

/// Validate max_tokens parameter (1 to 100000)
pub fn validate_max_tokens(max_tokens: u32) -> Result<(), ValidationError> {
    validate_range("max_tokens", max_tokens, 1, 100_000)
}

/// Validate project/conversation name (1-200 chars, no special chars)
pub fn validate_name(field: &str, name: &str) -> Result<(), ValidationError> {
    validate_not_empty(field, name)?;
    validate_length(field, name, Some(1), Some(200))?;

    // Check for potentially dangerous characters (basic sanitization)
    if name.contains('\0') || name.contains('\r') || name.contains('\n') {
        return Err(ValidationError::InvalidCharacters {
            field: field.to_string(),
        });
    }

    Ok(())
}

/// Validate document content (not empty, max 10MB)
pub fn validate_document_content(content: &str) -> Result<(), ValidationError> {
    validate_not_empty("content", content)?;
    validate_length("content", content, Some(1), Some(10_485_760))?; // 10MB limit
    Ok(())
}

/// Validate query string (not empty, max 10000 chars)
pub fn validate_query(query: &str) -> Result<(), ValidationError> {
    validate_not_empty("query", query)?;
    validate_length("query", query, Some(1), Some(10_000))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_not_empty() {
        assert!(validate_not_empty("test", "hello").is_ok());
        assert!(validate_not_empty("test", "").is_err());
        assert!(validate_not_empty("test", "   ").is_err());
    }

    #[test]
    fn test_validate_length() {
        assert!(validate_length("test", "hello", Some(1), Some(10)).is_ok());
        assert!(validate_length("test", "hello", Some(10), None).is_err());
        assert!(validate_length("test", "hello", None, Some(3)).is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range("test", 5, 1, 10).is_ok());
        assert!(validate_range("test", 0, 1, 10).is_err());
        assert!(validate_range("test", 11, 1, 10).is_err());
    }

    #[test]
    fn test_validate_name() {
        assert!(validate_name("name", "My Project").is_ok());
        assert!(validate_name("name", "").is_err());
        assert!(validate_name("name", "test\0name").is_err());
    }
}
