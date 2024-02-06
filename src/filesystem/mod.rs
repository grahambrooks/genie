use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub struct FileSystemContext<'a> {
    root: &'a Path,
}

impl FileSystemContext<'_> {
    pub fn new(root: &Path) -> FileSystemContext {
        FileSystemContext { root }
    }

    pub fn context(&self) -> Vec<PathBuf> {
        WalkDir::new(self.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .collect()
    }
}

fn path_to_string(root: &Path, path: &Path, isDir: bool) -> String {
    if path == root {
        return format!("/{}/", root.file_name().unwrap().to_string_lossy());
    }

    let components = path.components()
        .skip(root.components().count())
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>();

    if components.is_empty() {
        return String::new();
    }

    let mut result = String::from("    ");
    for (i, component) in components.iter().enumerate() {
        if i != components.len() - 1 {
            result.push_str("    ");
        } else {
            if isDir {
                result.push_str("/");
            }
            result.push_str(component);
            if isDir {
                result.push_str("/");
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_path_to_string() {
        let root = Path::new("/root");

        assert_eq!(path_to_string(root, root, true), "/root/");
        assert_eq!(path_to_string(root, Path::new("/root/file.txt"), false), "    file.txt");
        assert_eq!(path_to_string(root, Path::new("/root/src/"), true), "    /src/");
        assert_eq!(path_to_string(root, Path::new("/root/src/main.py"), false), "        main.py");
    }

    #[test]
    fn test_walk_dir() {
        let paths = FileSystemContext::new(Path::new(".")).context();
        assert!(paths.len() > 0);
    }

    #[test]
    fn test_walk_dir_not_found() {
        let paths = FileSystemContext::new(Path::new("not_found"))
            .context();
        assert_eq!(paths.len(), 0);
    }

    #[test]
    fn test_walk_dir_not_dir() {
        let paths = FileSystemContext::new(Path::new("Cargo.toml")).context();
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn test_walk_dir_nested() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = temp_dir.path();
        let temp_file_path = temp_dir_path.join("temp_file");
        fs::File::create(&temp_file_path).unwrap();

        let paths = FileSystemContext::new(&temp_dir_path).context();

        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_walk_dir_nested_dir() {
        let structure = "
/root_directory/
    /src/                 # Source files
        main.py           # Main application script
        helper.py         # Helper functions
    /docs/                # Documentation files
        README.md
        CONTRIBUTING.md
    /tests/               # Test scripts
        test_main.py
        test_helper.py
    requirements.txt      # Project dependencies
";
    }
}
