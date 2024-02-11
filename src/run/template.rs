use std::path::Path;

use crate::filesystem::FileSystemContext;
use crate::run::file_extensions;

pub fn expand_template(template: String, filepath: String) -> String {
    let mut result = template.to_string();
    if result.contains("{file.path}") {
        let path = std::path::Path::new(&filepath).parent().unwrap().to_str().unwrap();
        result = result.replace("{file.path}", path);
    }
    if result.contains("{file.name}") {
        let filename = std::path::Path::new(&filepath).file_name().unwrap().to_str().unwrap();
        result = result.replace("{file.name}", filename);
    }

    // generate a markdown content type from the filepath extension
    if result.contains("{file.type}") {
        let extension = std::path::Path::new(&filepath).extension().unwrap().to_str().unwrap();
        match file_extensions::get_language_from_extension(extension) {
            Some(language) => {
                result = result.replace("{file.type}", language);
            }
            None => {
                panic!("Unknown file type {}", extension);
            }
        }
    }
    if result.contains("{file.content}") {
        let content = std::fs::read_to_string(filepath.clone()).unwrap();
        result = result.replace("{file.content}", &content);
    }

    if result.contains("{project.path}") {
        result = result.replace("{project.path}", filepath.as_str());
    }

    if result.contains("{project.context}") {
        result = result.replace("{project.context}", FileSystemContext::new(Path::new(filepath.as_str())).context().as_str());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_template() {
        let template = "This is a {file.type} file with the name {file.name} and the content {file.content}".to_string();
        let filepath = "src/run/template.rs".to_string();
        let result = expand_template(template, filepath);
        assert_eq!(result.contains("Rust"), true);
        assert_eq!(result.contains("template.rs"), true);
        assert_eq!(result.contains("use crate::run;"), true);
    }

    #[test]
    fn test_project_context_expansion() {
        let template = "{project.path}/recommendations.md";
        let project_path = "src".to_string();
        let result = expand_template(template.to_string(), project_path);
        assert_eq!(result, "src/recommendations.md");
    }
}
