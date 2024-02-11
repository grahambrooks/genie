use std::error::Error;
use std::fmt;

use crate::adapters;
use crate::adapters::Adapter;
use crate::errors::GenieError;

pub(crate) struct Model {
    protocol: String,
    model_name: String,
}

pub(crate) trait ShellExecutor {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}


impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Protocol: {}, Model Name: {}", self.protocol, self.model_name)
    }
}

impl Model {
    #[allow(dead_code)]
    pub(crate) fn new(protocol: &'static str, model_name: &'static str) -> Self {
        Self {
            protocol: protocol.to_string(),
            model_name: model_name.to_string(),
        }
    }
    pub(crate) fn from_string(spec: &str) -> Result<Self, Box<dyn Error>> {
        let mut split_spec = spec.split("::");

        let protocol = split_spec.next();
        if protocol.is_none() {
            return Err(Box::new(GenieError::new(&format!("Error parsing model: {}", spec))));
        }
        let model_name = split_spec.next();
        if model_name.is_none() {
            return Err(Box::new(GenieError::new(&format!("Error parsing model: {}", spec))));
        }
        Ok(Self {
            protocol: protocol.unwrap().to_string(),
            model_name: model_name.unwrap().to_string(),
        })
    }

    pub(crate) fn adapter(&self) -> Result<Box<dyn Adapter>, Box<dyn Error>> {
        match self.protocol.as_str() {
            "ollama" => Ok(Box::new(adapters::ollama::OllamaChat::new(self.model_name.clone()))),
            "openai" => Ok(Box::new(adapters::openai::OpenAIGPTChat::new(self.model_name.clone()))),
            _ => Err(Box::new(GenieError::new(&format!("Error parsing model: {}", self.protocol))))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_from_string() {
        let model = Model::from_string("ollama::gpt-3").unwrap();
        assert_eq!(model.protocol, "ollama");
        assert_eq!(model.model_name, "gpt-3");
    }

    #[test]
    fn test_model_from_string_failure() {
        let model = Model::from_string("ollama:gpt-3");
        assert!(model.is_err());
    }


    #[test]
    fn test_model_adapter() {
        let model = Model::from_string("ollama::gpt-3").unwrap();
        let adapter = model.adapter();
        assert!(adapter.is_ok());
    }

    #[test]
    fn test_model_adapter_error() {
        let model = Model::from_string("unknown::gpt-3").unwrap();
        let adapter = model.adapter();
        assert!(adapter.is_err());
    }
}
