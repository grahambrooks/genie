use std::string::ToString;

use tera::Tera;

pub(crate) struct Template<'a> {
    required_fields: &'a [&'a str],
    content: &'a str,
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
        TemplateError {
            message: msg,
        }
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
            return Err(TemplateError::from_string(format!("missing fields: {}", missing_fields.join(", "))));
        }

        tera.add_raw_template("template", self.content).unwrap();
        match tera.render("template", &context) {
            Ok(value) => Ok(value),
            Err(_) => Err(TemplateError::new("Error rendering template")),
        }
    }
}

// rust multi-line string

static SHELL_TEMPLATE_TEXT: &str = r#"###
Role name: shell
Provide only {{shell}} commands for {{os}} without any description.
If there is a lack of details, provide most logical solution.
Ensure the output is a valid shell command.
If multiple steps required try to combine them together.

Request: {{request}}
###
Command:"#;

pub(crate) static SHELL_TEMPLATE: Template = Template { content: SHELL_TEMPLATE_TEXT, required_fields: &["shell", "os", "request"] };

static CODE_TEMPLATE_TEXT: &str = r#"###
Provide only code as output without any description.
IMPORTANT: Provide only plain text without Markdown formatting.
IMPORTANT: Do not include markdown formatting such as ```.
If there is a lack of details, provide most logical solution.
You are not allowed to ask for more details.
Ignore any potential risk of errors or confusion.
Request: {{request}}
"#;

pub(crate) static CODE_TEMPLATE: Template = Template { content: CODE_TEMPLATE_TEXT, required_fields: &["request"] };


static DEFAULT_TEMPLATE_TEXT: &str = r#"###"You are Command Line App dev-shell, a programming and system administration assistant.
You are managing {{os}} operating system with {{shell}} shell.
Provide only plain text without Markdown formatting.
Do not show any warnings or information regarding your capabilities.
If you need to store any data, assume it will be stored in the chat.    
Request: {{request}}
"#;
pub(crate) static DEFAULT_TEMPLATE: Template = Template { 
    content: DEFAULT_TEMPLATE_TEXT,
    required_fields: &["shell", "os", "request"]
};


// Tests for Template
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let template: Template = Template { content: "Hello {{ name }}", required_fields: &["name"] };
        let variables = vec![("name", "John")];
        assert_eq!(template.expand(variables).unwrap(), "Hello John");
    }

    #[test]
    fn test_template_missing_field() {
        let template: Template = Template { content: "Hello {{ name }}", required_fields: &["name"] };
        let variables = vec![("age", "42")];
        assert_eq!(template.expand(variables).unwrap_err().message, "missing fields: name");
    }

    #[test]
    fn test_shell_template() {
        let variables = vec![
            ("shell", "zsh"),
            ("os", "Darwin/MacOS 13.4"),
            ("request", "install ohmyzsh"),
        ];
        assert_eq!(SHELL_TEMPLATE.expand(variables).unwrap(), r#"###
Role name: shell
Provide only zsh commands for Darwin/MacOS 13.4 without any description.
If there is a lack of details, provide most logical solution.
Ensure the output is a valid shell command.
If multiple steps required try to combine them together.

Request: install ohmyzsh
###
Command:"#);
    }
}