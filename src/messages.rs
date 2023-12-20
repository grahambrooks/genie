pub(crate) mod template;

static SHELL_TEMPLATE_TEXT: &str = r#"###Role name: shell
Provide only {{shell}} commands for {{os}} without any description.
If there is a lack of details, provide most logical solution.
Ensure the output is a valid shell command.
If multiple steps required try to combine them together.
Do not output any other text or markdown formatting.
Generate output that can be piped into the shell through stdin
Request: {{request}}
###
Command:"#;

pub(crate) static SHELL_TEMPLATE: template::Template = template::Template {
    content: SHELL_TEMPLATE_TEXT,
    required_fields: &["shell", "os", "request"],
};

static CODE_TEMPLATE_TEXT: &str = r#"###Provide only code as output without any description.
IMPORTANT: Provide only plain text without Markdown formatting.
IMPORTANT: Do not include markdown formatting such as ```.
If there is a lack of details, provide most logical solution.
You are not allowed to ask for more details.
Ignore any potential risk of errors or confusion.
Request: {{request}}
"#;

pub(crate) static CODE_TEMPLATE: template::Template = template::Template {
    content: CODE_TEMPLATE_TEXT,
    required_fields: &["request"],
};

static DEFAULT_TEMPLATE_TEXT: &str = r#"###"You are Command Line App genie, a programming and system administration assistant.
You are managing {{os}} operating system with {{shell}} shell.
Provide only plain text without Markdown formatting.
Do not show any warnings or information regarding your capabilities.
If you need to store any data, assume it will be stored in the chat.
Request: {{request}}
"#;
pub(crate) static DEFAULT_TEMPLATE: template::Template = template::Template {
    content: DEFAULT_TEMPLATE_TEXT,
    required_fields: &["shell", "os", "request"],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_template() {
        let variables = vec![
            ("shell", "zsh"),
            ("os", "Darwin/MacOS 13.4"),
            ("request", "install ohmyzsh"),
        ];

        let expanded = SHELL_TEMPLATE.expand(variables).unwrap();
        assert!(expanded.contains("install ohmyzsh"));
        assert!(expanded.contains("Darwin/MacOS 13.4"));
        assert!(expanded.contains("zsh"));
    }
}
