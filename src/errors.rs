use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub(crate) struct GenieError {
    details: String,
}

impl GenieError {
    pub(crate) fn new(msg: &str) -> GenieError {
        GenieError { details: msg.to_string() }
    }
}

impl fmt::Display for GenieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GenieError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::GenieError;

    #[test]
    fn test_genie_error() {
        let error_message = "Test error message";
        let error = GenieError::new(error_message);

        assert_eq!(error.details, error_message);
    }

    fn test_method() -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(GenieError::new("Test error message")))
    }

    #[test]
    fn test_genie_error_from_method() {
        let result = test_method();

        assert!(result.is_err());
    }
}
