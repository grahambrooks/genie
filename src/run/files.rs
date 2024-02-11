pub fn get_matching_files(pattern: &str) -> Result<Vec<String>, glob::GlobError> {
    let mut result = Vec::new();
    match glob::glob(pattern) {
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(path) => result.push(path.display().to_string()),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
    Ok(result)
}
