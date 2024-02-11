use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::run::files;
use crate::run::template::expand_template;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum Source {
    Filesystem { path: Option<String> },
    Project { path: Option<String> },
}

// Implement an iterator for Source enum
impl Source {
    pub fn items(&self) -> Vec<String> {
        match self {
            Source::Filesystem { path } => {
                files::get_matching_files(path.clone().unwrap().as_str()).unwrap()
            }
            Source::Project { path } => {
                vec![path.clone().unwrap()]
            }
        }
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum Sink {
    Filesystem { path: Option<String> },
    Project { file: Option<String> },
}

pub(crate) trait SinkTarget {
    fn write(&self, name: String, content: String) -> Result<(), Box<dyn Error>>;
}

// Implement SinkTarget for Sink and enum Sink::filesystem
impl SinkTarget for Sink {
    fn write(&self, name: String, content: String) -> Result<(), Box<dyn Error>> {
        match self {
            Sink::Filesystem { path } => {
                let output_path = expand_template(path.clone().unwrap().to_string(), name);
                println!("\x1b[31mwriting {}\x1b[0m", output_path);

                // Write content to file
                let mut file = File::create(output_path)?;
                file.write_all(content.as_bytes())?;
            }
            Sink::Project { file } => {
                let output_path = expand_template(file.clone().unwrap().to_string(), name);
                println!("\x1b[31mwriting {}\x1b[0m", output_path);

                // Write content to file
                let mut file = File::create(output_path)?;
                file.write_all(content.as_bytes())?;
            }
        }
        Ok(())
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Task {
    pub(crate) template: Option<String>,
    #[serde(default = "default_model")]
    pub(crate) model: String,
    pub(crate) source: Source,
    pub(crate) sink: Sink,
}

fn default_model() -> String {
    "ollama:mistral:latest".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    pub(crate) task: Task,
}

pub fn parse_script_file(file_path: &str) -> Result<Script, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    parse_script(&mut file)
}

fn parse_script(source: &mut dyn io::Read) -> Result<Script, Box<dyn Error>> {
    let script: Script = serde_yaml::from_reader(source)?;

    if script.task.template.is_none() {
        return Err("task.template is required and currently missing".into());
    }

    // match on source enum
    match &script.task.source {
        Source::Filesystem { path } => {
            if path.is_none() {
                return Err("task.source.path is required and currently missing".into());
            }
        }
        Source::Project { path } => {
            if path.is_none() {
                return Err("task.source.path is required and currently missing for project sources".into());
            }
        }
    }

    match &script.task.sink {
        Sink::Filesystem { path } => {
            if path.is_none() {
                return Err("task.sink.path is required and currently missing".into());
            }
        }
        Sink::Project { file } => {
            if file.is_none() {
                return Err("task.sink.file is required and currently missing for project sinks".into());
            }
        }
    }

    Ok(script)
}

// Test deserializing scripts with validation of field values
// example in simple-script.yaml
#[cfg(test)]
mod tests {
    use super::*;

    // Test for parse script
    #[test]
    fn test_parse_script() {
        // language=yaml
        let script = r#"
        task:
          template: "This is a {file.type} file with the name {file.name} and the content {file.content}"
          model: "simple"
          source:
            type: "Filesystem"
            path: "src/run/template.rs"
          sink:
            type: "Filesystem"
            path: "output.txt"
        "#;
        let mut file = io::Cursor::new(script);

        let result = parse_script(&mut file).unwrap();
        assert_eq!(result.task.template.unwrap(), "This is a {file.type} file with the name {file.name} and the content {file.content}");
        assert_eq!(result.task.model, "simple");
        match result.task.source {
            Source::Filesystem { path } => {
                assert_eq!(path.unwrap(), "src/run/template.rs");
            }
            Source::Project { path } => {
                panic!("Expected Source::Filesystem, got Source::Project with path {:?}", path);
            }
        }
        match result.task.sink {
            Sink::Filesystem { path } => {
                assert_eq!(path.unwrap(), "output.txt");
            }
            Sink::Project { file } => {
                panic!("Expected Sink::Filesystem, got Sink::Project with file {:?}", file);
            }
        }
    }

    #[test]
    fn test_parse_script_no_model() {
        // language=yaml
        let script = r#"
        task:
          template: "This is a {file.type} file with the name {file.name} and the content {file.content}"
          source:
            type: Filesystem
            path: "src/run/template.rs"
          sink:
            type: "Filesystem"
            path: "output.txt"
        "#;
        let mut file = io::Cursor::new(script);

        let result = parse_script(&mut file).unwrap();
        assert_eq!(result.task.template.unwrap(), "This is a {file.type} file with the name {file.name} and the content {file.content}");
        assert_eq!(result.task.model, "ollama:mistral:latest");
        match result.task.source {
            Source::Filesystem { path } => {
                assert_eq!(path.unwrap(), "src/run/template.rs");
            }
            Source::Project { path } => {
                panic!("Expected Source::Filesystem, got Source::Project with path {:?}", path);
            }
        }
        match result.task.sink {
            Sink::Filesystem { path } => {
                assert_eq!(path.unwrap(), "output.txt");
            }
            Sink::Project { file } => {
                panic!("Expected Sink::Filesystem, got Sink::Project with file {:?}", file);
            }
        }
    }

    #[test]
    fn test_parse_script_no_template() {
        // language=yaml
        let script = r#"
        task:
          source:
            type: Filesystem
            path: "src/run/template.rs"
          sink:
            type: "Filesystem"
            path: "output.txt"
        "#;
        let mut file = io::Cursor::new(script);

        let result = parse_script(&mut file);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_script_no_source_path() {
        // language=yaml
        let script = r#"
        task:
          template: "This is a {file.type} file with the name {file.name} and the content {file.content}"
          source:
            type: Filesystem
          sink:
            type: "Filesystem"
            path: "output.txt"
        "#;
        let mut file = io::Cursor::new(script);

        let result = parse_script(&mut file);
        assert!(result.is_err());
    }

    #[test]
    fn test_parses_project_anaysis() {
        // language=yaml
        let script = r#"task:
  template: |
    analyze project structure
    {project.path}
    ```
    {project.context}
    ```
  model: 'ollama::mistral:latest'
  source:
    type: Project
    path: "src"
  sink:
    type: Project
    file: "{project.path}/recommendations.md"
"#;
        let mut file = io::Cursor::new(script);

        let result = parse_script(&mut file).unwrap();
        assert_eq!(result.task.template.unwrap(), "analyze project structure\n{project.path}\n```\n{project.context}\n```\n");
        assert_eq!(result.task.model, "ollama::mistral:latest");
        match result.task.source {
            Source::Filesystem { path } => {
                panic!("Expected Source::Project, got Source::Filesystem with path {:?}", path);
            }
            Source::Project { path } => {
                assert_eq!(path.unwrap(), "src");
            }
        }
        match result.task.sink {
            Sink::Filesystem { path } => {
                panic!("Expected Sink::Project, got Sink::Filesysten with path {:?}", path);
            }
            Sink::Project { file } => {
                assert_eq!(file.unwrap(), "{project.path}/recommendations.md");
            }
        }
    }
}