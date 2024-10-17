use std::string::ToString;

use tera::Tera;

pub(crate) struct Template<'a> {
    pub(crate) required_fields: &'a [&'a str],
    pub(crate) content: &'a str,
}

type TemplateResult<T> = Result<T, TemplateError>;

#[derive(Debug, Clone)]
pub(crate) struct TemplateError {
    pub message: String,
}

impl TemplateError {
    fn new(msg: &str) -> Self {
        TemplateError {
            message: msg.to_string(),
        }
    }
    fn from_string(msg: String) -> Self {
        TemplateError { message: msg }
    }
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sample error: {}", self.message)
    }
}

impl std::error::Error for TemplateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl<'a> Template<'a> {
    pub(crate) fn expand(&self, variables: Vec<(&str, &str)>) -> TemplateResult<String> {
        let mut missing_fields = Vec::from(self.required_fields);
        let mut tera = Tera::default();
        let mut context = tera::Context::new();
        for (key, value) in variables {
            missing_fields.retain(|value| *value != key);
            context.insert(key, &value);
        }

        if !missing_fields.is_empty() {
            return Err(TemplateError::from_string(format!(
                "missing fields: {}",
                missing_fields.join(", ")
            )));
        }

        tera.add_raw_template("template", self.content.replace("{", "{{").replace("}", "}}").as_str()).unwrap();
        match tera.render("template", &context) {
            Ok(value) => Ok(value.replace("{{", "{").replace("}}", "}")),
            Err(_) => Err(TemplateError::new("Error rendering template")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let template: Template = Template {
            content: "Hello {name}",
            required_fields: &["name"],
        };
        let variables = vec![("name", "John")];
        assert_eq!(template.expand(variables).unwrap(), "Hello John");
    }

    #[test]
    fn test_template_missing_field() {
        let template: Template = Template {
            content: "Hello {name}",
            required_fields: &["name"],
        };
        let variables = vec![("age", "42")];
        assert_eq!(
            template.expand(variables).unwrap_err().message,
            "missing fields: name"
        );
    }
}
