pub fn get_language_from_extension(extension: &str) -> Option<&'static str> {
    match extension {
        "rs" => Some("Rust"),
        "js" => Some("JavaScript"),
        "py" => Some("Python"),
        "java" => Some("Java"),
        "c" => Some("C"),
        "cpp" => Some("C++"),
        "go" => Some("Go"),
        "rb" => Some("Ruby"),
        "php" => Some("PHP"),
        "swift" => Some("Swift"),
        "cs" => Some("C#"),
        "sql" => Some("SQL"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::run::file_extensions::get_language_from_extension;

    #[test]
    fn test_get_language_from_extension() {
        assert_eq!(get_language_from_extension("rs"), Some("Rust"));
        assert_eq!(get_language_from_extension("js"), Some("JavaScript"));
        assert_eq!(get_language_from_extension("py"), Some("Python"));
        assert_eq!(get_language_from_extension("java"), Some("Java"));
        assert_eq!(get_language_from_extension("c"), Some("C"));
        assert_eq!(get_language_from_extension("cpp"), Some("C++"));
        assert_eq!(get_language_from_extension("go"), Some("Go"));
        assert_eq!(get_language_from_extension("rb"), Some("Ruby"));
        assert_eq!(get_language_from_extension("php"), Some("PHP"));
        assert_eq!(get_language_from_extension("swift"), Some("Swift"));
        assert_eq!(get_language_from_extension("cs"), Some("C#"));
        assert_eq!(get_language_from_extension("unknown"), None);
    }
}
