use std::error::Error;
use std::fs;
use std::path::Path;

use walkdir::WalkDir;

pub struct FileSystemContext<'a> {
    root: &'a Path,
}

impl FileSystemContext<'_> {
    pub fn new(root: &Path) -> FileSystemContext {
        FileSystemContext { root }
    }

    pub fn context(&self) -> String {
        if !self.root.exists() {
            return String::new();
        }
        let root_path = self.root;
        let mut result = String::new();
        result.push_str("/");
        result.push_str(root_path.components().last().unwrap().as_os_str().to_str().unwrap());
        // let root_depth = root_path.components().count();

        for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
            let depth = entry.depth();// - root_depth;
            let indent = "    ".repeat(depth);
            let path = entry.path();
            let display_name = path.strip_prefix(root_path).unwrap_or(path).to_str().unwrap();

            if entry.file_type().is_dir() {
                result.push_str(&format!("{}{}/\n", indent, display_name));
            } else {
                result.push_str(&format!("{}{}\n", indent, display_name));
            }
        }

        result
    }
}


#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

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
    fn test_walk_dir_nested_dir() {
        let structure = "/root_directory/
    requirements.txt
    tests/
        tests/test_helper.py
        tests/test_main.py
    docs/
        docs/README.md
        docs/CONTRIBUTING.md
    src/
        src/helper.py
        src/main.py
";
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = temp_dir.path();
        let root_directory = temp_dir_path.join("root_directory");
        fs::create_dir(&root_directory).unwrap();
        let src = root_directory.join("src");
        fs::create_dir(&src).unwrap();
        let main_py = src.join("main.py");
        fs::File::create(&main_py).unwrap();
        let helper_py = src.join("helper.py");
        fs::File::create(&helper_py).unwrap();
        let docs = root_directory.join("docs");
        fs::create_dir(&docs).unwrap();
        let readme_md = docs.join("README.md");
        fs::File::create(&readme_md).unwrap();
        let contributing_md = docs.join("CONTRIBUTING.md");
        fs::File::create(&contributing_md).unwrap();
        let tests = root_directory.join("tests");
        fs::create_dir(&tests).unwrap();
        let test_main_py = tests.join("test_main.py");
        fs::File::create(&test_main_py).unwrap();
        let test_helper_py = tests.join("test_helper.py");
        fs::File::create(&test_helper_py).unwrap();
        let requirements_txt = root_directory.join("requirements.txt");
        fs::File::create(&requirements_txt).unwrap();

        let paths = FileSystemContext::new(&root_directory).context();

        println!("{}", paths);

        assert_eq!(paths, structure);
    }
}
